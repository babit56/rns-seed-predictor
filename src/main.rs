use clap::{Args, Parser};
use rayon::prelude::*;
use rns_seed_predictor::{types::Unlocks, Run};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    num::ParseIntError,
    path::PathBuf,
};

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about,
    after_help = "If no unlock-related flags are provided, all sets are assumed to be unlocked. If only single set args are given, all sets except the given ones are assumed to be locked.
The order of priority for unlock-related flags is 1. --unlock-bitstring, 2. --no-unlocks and 3. specific sets"
)]
struct Cli {
    /// Optional seed to generate. If not specificed, run for all seeds
    #[arg(short, long)]
    seed: Option<u32>,

    /// How many players to simulate for. Must be 1-4
    #[arg(short, long, default_value_t = 4)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=4))]
    players: u8,

    /// Generate for Cute/Normal instead of Hard/Lunar
    #[arg(short, long)]
    easy: bool,

    /// Turn off all unlocks. Incompatiable (= ignored)
    #[arg(long, conflicts_with_all = ["single_unlocks", "unlock_bitstring"])]
    no_unlocks: bool,

    /// Specify unlocks with a bitstring like 1100010001, where darkbite is the rightmost bit, timegem the 2nd, etc. Uses the same order as seen on the wiki. useful for debugging --full-generation output
    #[arg(long, value_parser = parse_bitstring, conflicts_with = "single_unlocks")]
    unlock_bitstring: Option<usize>,

    #[command(flatten)]
    single_unlocks: SingleUnlocks,

    /// The file to write to when generating a single csv file
    #[arg(short, long, default_value = "unique_seeds.csv")]
    outfile: PathBuf,

    /// Instead of generating seeds for the specified unlocks, generate seeds for all unlock combinations (where the specificed unlocks are indeed unlocked).
    /// Seeds are written to a file named after their respective unlock bitstring, in the folder full_gen/
    #[arg(long, conflicts_with = "seed")]
    full_generation: bool,
}

#[derive(Args, Debug, Clone)]
struct SingleUnlocks {
    #[arg(long)]
    darkbite: bool,

    #[arg(long)]
    timegem: bool,

    #[arg(long)]
    youkai: bool,

    #[arg(long)]
    haunted: bool,

    #[arg(long)]
    gladiator: bool,

    #[arg(long)]
    sparkblade: bool,

    #[arg(long)]
    swiftflight: bool,

    #[arg(long)]
    sacredflame: bool,

    #[arg(long)]
    ruins: bool,

    #[arg(long)]
    lakeshrine: bool,
}

impl Cli {
    #[rustfmt::skip]
    fn get_unlocks(self: &Self) -> Unlocks {
        let mut unlocks = Unlocks::with_none();

        if self.single_unlocks.darkbite    { unlocks.darkbite    = true; }
        if self.single_unlocks.timegem     { unlocks.timegem     = true; }
        if self.single_unlocks.youkai      { unlocks.youkai      = true; }
        if self.single_unlocks.haunted     { unlocks.haunted     = true; }
        if self.single_unlocks.gladiator   { unlocks.gladiator   = true; }
        if self.single_unlocks.sparkblade  { unlocks.sparkblade  = true; }
        if self.single_unlocks.swiftflight { unlocks.swiftflight = true; }
        if self.single_unlocks.sacredflame { unlocks.sacredflame = true; }
        if self.single_unlocks.ruins       { unlocks.ruins       = true; }
        if self.single_unlocks.lakeshrine  { unlocks.lakeshrine  = true; }

        if let Some(bitstring) = self.unlock_bitstring {
            Unlocks::from_bitstring(bitstring)
        } else if self.no_unlocks {
            Unlocks::with_none()
        } else if unlocks == Unlocks::with_none() {
            Unlocks::with_all()
        } else {
            unlocks
        }
    }
}

fn parse_bitstring(bitstring: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(bitstring, 2)
}

// Get enough state to tell completely whether two seeds will be equal
// I.e. calculate the first run of the TLCG for seed and seed+5
fn get_short_state(seed: u32) -> (u32, u32) {
    (
        (seed.wrapping_mul(0x343fd).wrapping_add(0x269ec3)) >> 16,
        (seed
            .wrapping_add(5)
            .wrapping_mul(0x343fd)
            .wrapping_add(0x269ec3))
            >> 16,
    )
}

fn get_unique_seeds() -> Vec<u32> {
    // NOTE: assumes there are 2^17 unique seeds
    let unique_seeds = 2usize.pow(17);
    let mut states: Vec<(u32, u32)> = Vec::with_capacity(unique_seeds);
    let mut seeds: Vec<u32> = Vec::with_capacity(unique_seeds);
    for seed in 0.. {
        // Check if we have found all seeds
        if states.len() == unique_seeds {
            break;
        }
        // Check if this is a new unique seed
        let short_state = get_short_state(seed);
        if states.contains(&short_state) {
            continue;
        }
        states.push(short_state);
        seeds.push(seed);
    }
    seeds
}

fn generate_csv(players: u8, high_difficulty: bool, unlocks: Unlocks) -> Vec<String> {
    let seeds = get_unique_seeds();
    println!("Unique seeds enumerated, generating seeds");
    let unique_seeds = 2usize.pow(17);
    let mut lines: Vec<String> = Vec::with_capacity(unique_seeds);
    for seed in seeds {
        // Run prediction
        let mut run = Run::new(seed, players, high_difficulty, unlocks);
        run.predict_seed();
        lines.push(run.get_csv_line());

        if lines.len() % 10000 == 0 {
            println!(
                "{} out of {} seeds processed so far",
                lines.len(),
                unique_seeds
            );
        }
    }
    return lines;
}

fn full_generation(players: u8, high_difficulty: bool, unlock_mask: usize) {
    fs::create_dir_all("full_gen")
        .expect("Expected to be able to create directory 'full_gen/' for saving csv's");
    let seeds = get_unique_seeds();
    println!("Unique seeds enumerated, generating csv files");
    // Get unique combinations of Unlocks
    let unlock_combinations: Vec<Unlocks> = (0..2_usize.pow(10))
        .into_iter()
        .filter(|num| num & unlock_mask == unlock_mask)
        .map(|bitstring| Unlocks::from_bitstring(bitstring))
        .collect();
    // Generate csv file for each Unlocks
    unlock_combinations.into_par_iter().for_each(|unlocks| {
        let path = format!("full_gen/{:0>10b}.csv", unlocks.get_bitstring());
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        for seed in &seeds {
            let mut run = Run::new(*seed, players, high_difficulty, unlocks);
            run.predict_seed();
            writeln!(writer, "{}", run.get_short_line()).unwrap();
        }
    });
}

fn main() {
    let cli = Cli::parse();
    // println!("{:?}", cli);
    let unlocks = cli.get_unlocks();
    println!(
        "Using unlocks with following bitstring: {}",
        unlocks.get_bitstring()
    );
    if let Some(seed) = cli.seed {
        let mut run = Run::new(seed, cli.players, !cli.easy, unlocks);
        run.predict_seed();
        println!("{}", run);
        println!("{}", run.get_csv_line());
        println!("{}", run.get_short_line());
    } else if cli.full_generation {
        println!(
            "Generating csv's for all combinations of given unlocks, in total {} csv's",
            2_usize.pow(10 - unlocks.get_bitstring().count_ones())
        );
        let unlock_mask = cli.get_unlocks().get_bitstring();
        full_generation(cli.players, !cli.easy, unlock_mask);
    } else {
        println!("Generating one csv file");
        let csv_lines = generate_csv(cli.players, !cli.easy, unlocks);
        let mut file = File::create(&cli.outfile).expect(&format!(
            "Expected to be able to create file {}",
            cli.outfile.to_string_lossy()
        ));
        file.write_all(csv_lines.join("\n").as_bytes())
            .expect("Expected to be able to write to file unique_seeds.csv");
    }
}
