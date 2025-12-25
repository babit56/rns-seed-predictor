use clap::Parser;
use rayon::prelude::*;
use rns_seed_predictor::{types::Unlocks, Run};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about,
    after_help = "If no unlock-related flags are provided, all sets are assumed to be unlocked. If --no-unlocks is given, it gets priority over the other set-specific flags.
Otherwise, only the sets for the given set-flags are turned on"
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
    #[arg(long)]
    no_unlocks: bool,

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

    #[arg(short, long, default_value = "unique_seeds.csv")]
    outfile: PathBuf,

    /// Whether one specific combination of sets should be generated or if all possible combinations should be generated
    #[arg(long)]
    full_generation: bool,
}

impl Cli {
    #[rustfmt::skip]
    fn get_unlocks(self: &Self) -> Unlocks {
        let mut unlocks = Unlocks::with_none();

        if self.darkbite    { unlocks.darkbite    = true; }
        if self.timegem     { unlocks.timegem     = true; }
        if self.youkai      { unlocks.youkai      = true; }
        if self.haunted     { unlocks.haunted     = true; }
        if self.gladiator   { unlocks.gladiator   = true; }
        if self.sparkblade  { unlocks.sparkblade  = true; }
        if self.swiftflight { unlocks.swiftflight = true; }
        if self.sacredflame { unlocks.sacredflame = true; }
        if self.ruins       { unlocks.ruins       = true; }
        if self.lakeshrine  { unlocks.lakeshrine  = true; }

        if self.no_unlocks {
            Unlocks::with_none()
        } else if unlocks == Unlocks::with_none() {
            Unlocks::with_all()
        } else {
            unlocks
        }
    }
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
    fs::create_dir_all("full_gen").unwrap();
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
        let path = format!("full_gen/{:b}.csv", unlocks.get_bitstring());
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
    if let Some(seed) = cli.seed {
        let mut run = Run::new(seed, cli.players, !cli.easy, unlocks);
        run.predict_seed();
        println!("{}", run);
        println!("{}", run.get_csv_line());
    } else if cli.full_generation {
        let unlock_mask = cli.get_unlocks().get_bitstring();
        full_generation(cli.players, !cli.easy, unlock_mask);
    } else {
        let csv_lines = generate_csv(cli.players, !cli.easy, unlocks);
        let mut file = File::create(&cli.outfile).expect(&format!(
            "Expected to be able to create file {}",
            cli.outfile.to_string_lossy()
        ));
        file.write_all(csv_lines.join("\n").as_bytes())
            .expect("Expected to be able to write to file unique_seeds.csv");
    }
}
