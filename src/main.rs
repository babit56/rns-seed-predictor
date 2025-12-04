use std::{env, fmt, fs::File, io::Write};

const AREA_NAMES: [&'static str; 5] = [
    "hw_nest",
    "hw_arsenal",
    "hw_lighthouse",
    "hw_streets",
    "hw_lakeside",
];

const OUTSKIRT_NAMES: [[&'static str; 2]; 5] = [
    ["enc_bird_sophomore1", "enc_bird_sophomore2"],
    ["enc_frog_tinkerer1", "enc_frog_tinkerer2"],
    ["enc_dragon_granite1", "enc_dragon_granite2"],
    ["enc_wolf_blackear1", "enc_wolf_blackear2"],
    ["enc_mouse_cadet1", "enc_mouse_cadet2"],
];

const PALE_KEEP_NAMES: [&'static str; 5] = [
    "enc_queens_staff0",
    "enc_queens_knife0",
    "enc_queens_axe0",
    "enc_queens_spear0",
    "enc_queens_harp0",
];

const CHEST_NAMES: [&'static str; 6] = ["White", "Opal", "Sapphire", "Ruby", "Garnet", "Emerald"];

const OBJECT_NAMES: [&'static str; 524] = [
    "nothing_item",
    "???",
    "???",
    "???",
    "???",
    "???",
    "Dimi Moonburst",
    "Diamond Dimi Moonburst",
    "Opal Dimi Moonburst",
    "Sapphire Dimi Moonburst",
    "Ruby Dimi Moonburst",
    "Garnet Dimi Moonburst",
    "Emerald Dimi Moonburst",
    "Lat Moonburst",
    "Diamond Lat Moonburst",
    "Opal Lat Moonburst",
    "Sapphire Lat Moonburst",
    "Ruby Lat Moonburst",
    "Garnet Lat Moonburst",
    "Emerald Lat Moonburst",
    "Astral Swirl",
    "Diamond Astral Swirl",
    "Opal Astral Swirl",
    "Sapphire Astral Swirl",
    "Ruby Astral Swirl",
    "Garnet Astral Swirl",
    "Emerald Astral Swirl",
    "Astral Seal",
    "Diamond Astral Seal",
    "Opal Astral Seal",
    "Sapphire Astral Seal",
    "Ruby Astral Seal",
    "Garnet Astral Seal",
    "Emerald Astral Seal",
    "Swift Cut",
    "Diamond Swift Cut",
    "Opal Swift Cut",
    "Sapphire Swift Cut",
    "Ruby Swift Cut",
    "Garnet Swift Cut",
    "Emerald Swift Cut",
    "Knife Juggle",
    "Diamond Knife Juggle",
    "Opal Knife Juggle",
    "Sapphire Knife Juggle",
    "Ruby Knife Juggle",
    "Garnet Knife Juggle",
    "Emerald Knife Juggle",
    "Assassinate",
    "Diamond Assassinate",
    "Opal Assassinate",
    "Sapphire Assassinate",
    "Ruby Assassinate",
    "Garnet Assassinate",
    "Emerald Assassinate",
    "Shadowstep",
    "Diamond Shadowstep",
    "Opal Shadowstep",
    "Sapphire Shadowstep",
    "Ruby Shadowstep",
    "Garnet Shadowstep",
    "Emerald Shadowstep",
    "Full Moon",
    "Diamond Full Moon",
    "Opal Full Moon",
    "Sapphire Full Moon",
    "Ruby Full Moon",
    "Garnet Full Moon",
    "Emerald Full Moon",
    "Dark Wave",
    "Diamond Dark Wave",
    "Opal Dark Wave",
    "Sapphire Dark Wave",
    "Ruby Dark Wave",
    "Garnet Dark Wave",
    "Emerald Dark Wave",
    "Spinning Leap",
    "Diamond Spinning Leap",
    "Opal Spinning Leap",
    "Sapphire Spinning Leap",
    "Ruby Spinning Leap",
    "Garnet Spinning Leap",
    "Emerald Spinning Leap",
    "Shadow Barrier",
    "Diamond Shadow Barrier",
    "Opal Shadow Barrier",
    "Sapphire Shadow Barrier",
    "Ruby Shadow Barrier",
    "Garnet Shadow Barrier",
    "Emerald Shadow Barrier",
    "Twirling Rose",
    "Diamond Twirling Rose",
    "Opal Twirling Rose",
    "Sapphire Twirling Rose",
    "Ruby Twirling Rose",
    "Garnet Twirling Rose",
    "Emerald Twirling Rose",
    "Falling Petal",
    "Diamond Falling Petal",
    "Opal Falling Petal",
    "Sapphire Falling Petal",
    "Ruby Falling Petal",
    "Garnet Falling Petal",
    "Emerald Falling Petal",
    "Lily Blossom",
    "Diamond Lily Blossom",
    "Opal Lily Blossom",
    "Sapphire Lily Blossom",
    "Ruby Lily Blossom",
    "Garnet Lily Blossom",
    "Emerald Lily Blossom",
    "Golden Grace",
    "Diamond Golden Grace",
    "Opal Golden Grace",
    "Sapphire Golden Grace",
    "Ruby Golden Grace",
    "Garnet Golden Grace",
    "Emerald Golden Grace",
    "Dimi Life",
    "Diamond Dimi Life",
    "Opal Dimi Life",
    "Sapphire Dimi Life",
    "Ruby Dimi Life",
    "Garnet Dimi Life",
    "Emerald Dimi Life",
    "Lat Life",
    "Diamond Lat Life",
    "Opal Lat Life",
    "Sapphire Lat Life",
    "Ruby Lat Life",
    "Garnet Lat Life",
    "Emerald Lat Life",
    "Summon Spirit",
    "Diamond Summon Spirit",
    "Opal Summon Spirit",
    "Sapphire Summon Spirit",
    "Ruby Summon Spirit",
    "Garnet Summon Spirit",
    "Emerald Summon Spirit",
    "Nature's Blessing",
    "Diamond Nature's Blessing",
    "Opal Nature's Blessing",
    "Sapphire Nature's Blessing",
    "Ruby Nature's Blessing",
    "Garnet Nature's Blessing",
    "Emerald Nature's Blessing",
    "Sword Thrust",
    "Diamond Sword Thrust",
    "Opal Sword Thrust",
    "Sapphire Sword Thrust",
    "Ruby Sword Thrust",
    "Garnet Sword Thrust",
    "Emerald Sword Thrust",
    "Siphon Spell",
    "Diamond Siphon Spell",
    "Opal Siphon Spell",
    "Sapphire Siphon Spell",
    "Ruby Siphon Spell",
    "Garnet Siphon Spell",
    "Emerald Siphon Spell",
    "Magicked Fleche",
    "Diamond Magicked Fleche",
    "Opal Magicked Fleche",
    "Sapphire Magicked Fleche",
    "Ruby Magicked Fleche",
    "Garnet Magicked Fleche",
    "Emerald Magicked Fleche",
    "Novi Dark",
    "Diamond Novi Dark",
    "Opal Novi Dark",
    "Sapphire Novi Dark",
    "Ruby Novi Dark",
    "Garnet Novi Dark",
    "Emerald Novi Dark",
    "Arrowshot",
    "Diamond Arrowshot",
    "Opal Arrowshot",
    "Sapphire Arrowshot",
    "Ruby Arrowshot",
    "Garnet Arrowshot",
    "Emerald Arrowshot",
    "Rabbitsnare",
    "Diamond Rabbitsnare",
    "Opal Rabbitsnare",
    "Sapphire Rabbitsnare",
    "Ruby Rabbitsnare",
    "Garnet Rabbitsnare",
    "Emerald Rabbitsnare",
    "Barrage",
    "Diamond Barrage",
    "Opal Barrage",
    "Sapphire Barrage",
    "Ruby Barrage",
    "Garnet Barrage",
    "Emerald Barrage",
    "Careful Aim",
    "Diamond Careful Aim",
    "Opal Careful Aim",
    "Sapphire Careful Aim",
    "Ruby Careful Aim",
    "Garnet Careful Aim",
    "Emerald Careful Aim",
    "Flame Hook",
    "Diamond Flame Hook",
    "Opal Flame Hook",
    "Sapphire Flame Hook",
    "Ruby Flame Hook",
    "Garnet Flame Hook",
    "Emerald Flame Hook",
    "Quick Jab",
    "Diamond Quick Jab",
    "Opal Quick Jab",
    "Sapphire Quick Jab",
    "Ruby Quick Jab",
    "Garnet Quick Jab",
    "Emerald Quick Jab",
    "Fury",
    "Diamond Fury",
    "Opal Fury",
    "Sapphire Fury",
    "Ruby Fury",
    "Garnet Fury",
    "Emerald Fury",
    "Footwork",
    "Diamond Footwork",
    "Opal Footwork",
    "Sapphire Footwork",
    "Ruby Footwork",
    "Garnet Footwork",
    "Emerald Footwork",
    "Spear Thrust",
    "Diamond Spear Thrust",
    "Opal Spear Thrust",
    "Sapphire Spear Thrust",
    "Ruby Spear Thrust",
    "Garnet Spear Thrust",
    "Emerald Spear Thrust",
    "Lat Holy",
    "Diamond Lat Holy",
    "Opal Lat Holy",
    "Sapphire Lat Holy",
    "Ruby Lat Holy",
    "Garnet Lat Holy",
    "Emerald Lat Holy",
    "Novi Holy",
    "Diamond Novi Holy",
    "Opal Novi Holy",
    "Sapphire Novi Holy",
    "Ruby Novi Holy",
    "Garnet Novi Holy",
    "Emerald Novi Holy",
    "Roll & Phalanx",
    "Diamond Roll & Phalanx",
    "Opal Roll & Phalanx",
    "Sapphire Roll & Phalanx",
    "Ruby Roll & Phalanx",
    "Garnet Roll & Phalanx",
    "Emerald Roll & Phalanx",
    "Strike Command",
    "Diamond Strike Command",
    "Opal Strike Command",
    "Sapphire Strike Command",
    "Ruby Strike Command",
    "Garnet Strike Command",
    "Emerald Strike Command",
    "March Command",
    "Diamond March Command",
    "Opal March Command",
    "Sapphire March Command",
    "Ruby March Command",
    "Garnet March Command",
    "Emerald March Command",
    "Abyssal Call",
    "Diamond Abyssal Call",
    "Opal Abyssal Call",
    "Sapphire Abyssal Call",
    "Ruby Abyssal Call",
    "Garnet Abyssal Call",
    "Emerald Abyssal Call",
    "Protect Command",
    "Diamond Protect Command",
    "Opal Protect Command",
    "Sapphire Protect Command",
    "Ruby Protect Command",
    "Garnet Protect Command",
    "Emerald Protect Command",
    "???",
    "Raven Grimoire",
    "Blackwing Staff",
    "Curse Talon",
    "Darkmagic Blade",
    "Witch's Cloak",
    "Crowfeather Hairpin",
    "Redblack Ribbon",
    "Opal Necklace",
    "Sleeping Greatbow",
    "Crescentmoon Dagger",
    "Lullaby Harp",
    "Nightstar Grimoire",
    "Moon Pendant",
    "Pajama Hat",
    "Stuffed Rabbit",
    "Nightingale Gown",
    "Eternity Flute",
    "Timewarp Wand",
    "Chrome Shield",
    "Clockwork Tome",
    "Metronome Boots",
    "Timemage Cap",
    "Starry Cloak",
    "Gemini Necklace",
    "Hawkfeather Fan",
    "Windbite Dagger",
    "Pidgeon Bow",
    "Shinsoku Katana",
    "Eaglewing Charm",
    "Sparrow Feather",
    "Winged Cap",
    "Thief's Coat",
    "Vampiric Dagger",
    "Bloody Bandage",
    "Leech Staff",
    "Bloodhound Greatsword",
    "Reaper Cloak",
    "Bloodflower Brooch",
    "Wolf Hood",
    "Blood Vial",
    "Black Wakizashi",
    "Throwing Dagger",
    "Assassin's Knife",
    "Ninjutsu Scroll",
    "Shadow Bracelet",
    "Ninja Robe",
    "Kunoichi Hood",
    "Shinobi Tabi",
    "Dragonhead Spear",
    "Granite Greatsword",
    "Greysteel Shield",
    "Stonebreaker Staff",
    "Tough Gauntlet",
    "Rockdragon Mail",
    "Obsidian Hairpin",
    "Iron Greaves",
    "Volcano Spear",
    "Reddragon Blade",
    "Flame Bow",
    "Meteor Staff",
    "Phoenix Charm",
    "Firescale Corset",
    "Demon Horns",
    "Flamewalker Boots",
    "Diamond Shield",
    "Peridot Rapier",
    "Garnet Staff",
    "Sapphire Violin",
    "Emerald Chestplate",
    "Amethyst Bracelet",
    "Topaz Charm",
    "Ruby Circlet",
    "Brightstorm Spear",
    "Bolt Staff",
    "Lightning Bow",
    "Darkstorm Knife",
    "Darkcloud Necklace",
    "Crown of Storms",
    "Thunderclap Gloves",
    "Storm Petticoat",
    "Holy Greatsword",
    "Sacred Bow",
    "Purification Rod",
    "Ornamental Bell",
    "Shrinemaiden's Kosode",
    "Redwhite Ribbon",
    "Divine Mirror",
    "Golden Chime",
    "Book of Cheats",
    "Golden Katana",
    "Glittering Trumpet",
    "Royal Staff",
    "Ballroom Gown",
    "Silver Coin",
    "Queen's Crown",
    "Mimick Rabbitfoot",
    "Butterfly Ocarina",
    "Fairy Spear",
    "Moss Shield",
    "Floral Bow",
    "Blue Rose",
    "Sunflower Crown",
    "Midsummer Dress",
    "Grasswoven Bracelet",
    "Snakefang Dagger",
    "Ivy Staff",
    "Deathcap Tome",
    "Spiderbite Bow",
    "Compound Gloves",
    "Poisonfrog Charm",
    "Venom Hood",
    "Chemist's Coat",
    "Seashell Shield",
    "Necronomicon",
    "Tidal Greatsword",
    "Occult Dagger",
    "Mermaid Scalemail",
    "Hydrous Blob",
    "Abyss Artifact",
    "Lost Pendant",
    "Sawtooth Cleaver",
    "Raven's Dagger",
    "Killing Note",
    "Blacksteel Buckler",
    "Nightguard Gloves",
    "Sniper's Eyeglasses",
    "Darkmage Charm",
    "Firststrike Bracelet",
    "Obsidian Rod",
    "Darkglass Spear",
    "Timespace Dagger",
    "Quartz Shield",
    "Pocketwatch",
    "Nova Crown",
    "Blackhole Charm",
    "Twinstar Earrings",
    "Kyou No Omikuji",
    "Youkai Bracelet",
    "Oni Staff",
    "Kappa Shield",
    "Usagi Kamen",
    "Red Tanzaku",
    "Vega Spear",
    "Altair Dagger",
    "Ghost Spear",
    "Phantom Dagger",
    "Cursed Candlestaff",
    "Haunted Gloves",
    "Old Bonnet",
    "Maid Outfit",
    "Calling Bell",
    "Smoke Shield",
    "Grandmaster Spear",
    "Teacher Knife",
    "Tactician Rod",
    "Spiked Shield",
    "Battlemaiden Armor",
    "Gladiator Helmet",
    "Lancer Gauntlets",
    "Lion Charm",
    "Bluebolt Staff",
    "Lapis Sword",
    "Shockwave Tome",
    "Battery Shield",
    "Raiju Crown",
    "Staticshock Earrings",
    "Stormdance Gown",
    "Blackbolt Ribbon",
    "Crane Katana",
    "Falconfeather Dagger",
    "Tornado Staff",
    "Cloud Guard",
    "Hermes Bow",
    "Talon Charm",
    "Tiny Wings",
    "Feathered Overcoat",
    "Sandpriestess Spear",
    "Flamedancer Dagger",
    "Whiteflame Staff",
    "Sacred Shield",
    "Marble Clasp",
    "Sun Pendant",
    "Tiny Hourglass",
    "Desert Earrings",
    "Giant Stone Club",
    "Ruins Sword",
    "Mountain Staff",
    "Boulder Shield",
    "Golem's Claymore",
    "Stoneplate Armor",
    "Sacredstone Charm",
    "Clay Rabbit",
    "Waterfall Polearm",
    "Vorpal Dao",
    "Jade Staff",
    "Reflection Shield",
    "Butterfly Hairpin",
    "Watermage Pendant",
    "Raindrop Earrings",
    "Aquamarine Bracelet",
    "Full Heal",
    "Level Up",
    "Regeneration Potion",
    "Essence of Spell",
    "Darkness Potion",
    "Quickening Potion",
    "Winged Potion",
    "Essence of Wit",
    "Swifthand Potion",
    "Fire Potion",
    "Essence of Strength",
    "Golden Potion",
    "Luck Potion",
    "Essence of Steel",
    "Evasion Potion",
    "Longarm Potion",
    "Vitality Potion",
    "Opal Primary Upgrade",
    "Opal Secondary Upgrade",
    "Opal Special Upgrade",
    "Opal Defensive Upgrade",
    "Sapphire Primary Upgrade",
    "Sapphire Secondary Upgrade",
    "Sapphire Special Upgrade",
    "Sapphire Defensive Upgrade",
    "Ruby Primary Upgrade",
    "Ruby Secondary Upgrade",
    "Ruby Special Upgrade",
    "Ruby Defensive Upgrade",
    "Garnet Primary Upgrade",
    "Garnet Secondary Upgrade",
    "Garnet Special Upgrade",
    "Garnet Defensive Upgrade",
    "Emerald Primary Upgrade",
    "Emerald Secondary Upgrade",
    "Emerald Special Upgrade",
    "Emerald Defensive Upgrade",
];

#[allow(dead_code)]
#[derive(Debug)]
struct GMRand {
    seed: u32,
    state: [u32; 16],
    index: usize,
    random_poly: u32,
}

#[allow(dead_code)]
impl GMRand {
    // move seeding to separate function?
    // add option for "correct" seeding
    fn new() -> Self {
        GMRand {
            seed: 0,
            state: [0; 16],
            index: 0,
            random_poly: 0xDA442D24,
        }
    }

    // InitRandom in GML
    fn _init_random(self: &mut Self, seed: u32) -> u32 {
        self.seed = seed;
        self.index = 0;

        let mut temp = seed;
        for i in 0..16 {
            temp = (temp.wrapping_mul(0x343fd).wrapping_add(0x269ec3)) >> 16;
            self.state[i] = temp;
        }
        seed
    }

    // InitRandom but with correct usage of the TLCG
    // The TLCG is literally just the one from Lomont 2008, i.e. the MS algo for rand()
    fn _init_random_correct(self: &mut Self, seed: u32) -> u32 {
        self.seed = seed;

        let mut temp = seed;
        for i in 0..16 {
            temp = temp * 0x343fd + 0x269ec3;
            self.state[i] = (temp >> 16) & 0x7FFF;
        }
        seed
    }

    // YYRandom in GML source code
    fn _get_rand(self: &mut Self) -> u32 {
        // Taken from https://lomont.org/papers/2008/Lomont_PRNG_2008.pdf
        let (mut a, b, mut c, d);
        a = self.state[self.index];
        c = self.state[(self.index + 13) & 15];
        b = a ^ c ^ (a << 16) ^ (c << 15);
        c = self.state[(self.index + 9) & 15];
        c ^= c >> 11;
        self.state[self.index] = b ^ c;
        a = self.state[self.index];
        d = a ^ ((a << 5) & 0xDA442D24);
        self.index = (self.index + 15) & 15;
        a = self.state[self.index];
        self.state[self.index] = a ^ b ^ d ^ (a << 2) ^ (b << 18) ^ (c << 28);
        return self.state[self.index];

        // Adapted from rerand: https://gist.github.com/nkrapivin/ea4db4abb8a1994c0c1ba88f54196fa6
        // let t1 = self.state[self.index] ^ self.state[(self.index + 0xd) & 0xf];
        // let t2 =
        //     t1 ^ (self.state[self.index] << 0x10) ^ (self.state[(self.index + 0xd) & 0xf] << 0xf);
        // let t3 = self.state[(self.index + 9) & 0xf] ^ (self.state[(self.index + 9) & 0xf] >> 0xb);
        // let t4 = t2 ^ t3;
        // self.state[self.index] = t4;
        // self.index = (self.index - 1) & 0xf;
        // self.state[self.index] = self.state[self.index]
        //     ^ t2
        //     ^ (t4 ^ (self.random_poly & (t4 << 5)))
        //     ^ (self.state[self.index] << 2)
        //     ^ (t1 << 0x12)
        //     ^ (t3 << 0x1c);
        // self.state[self.index]
    }

    fn _get_rand_upper(self: &mut Self, upper: u32) -> u32 {
        self._get_rand() % upper
    }

    // iScript_Random in GML source code
    fn _get_rand_long(self: &mut Self, upper: i64) -> i64 {
        let rand1 = self._get_rand() as i64;
        let rand2 = self._get_rand() as i64;
        let wide_rand = rand1 | (rand2 << 32) & 0x7fffffffffffffff;
        if upper == 0 {
            wide_rand
        } else {
            upper.signum() * wide_rand % upper.abs()
        }
    }

    // fn iscript_random(self: &mut Self, upper: i64) -> i64 {
    //     let rand1 = self._get_rand() as i64;
    //     let rand2 = self._get_rand() as i64;
    //     let long_rand = (rand1 & 0xffffffff) | ((rand2 & 0x7fffffff) << 32);
    //     let mut signum = -1;
    //     if -1 < upper {
    //         // if upper is 0 or positive
    //         signum = 1
    //     }
    //     let nonneg_upper = signum * upper;
    //     if nonneg_upper == 0 {
    //         long_rand * signum
    //     } else {
    //         (long_rand - long_rand / nonneg_upper * nonneg_upper) * signum
    //     }
    // }

    fn get_seed(self: &Self) -> u32 {
        self.seed
    }

    fn set_seed(self: &mut Self, seed: u32) -> u32 {
        // floor input if float
        self._init_random(seed)
    }

    fn random(self: &mut Self, upper: f64) -> f64 {
        let rand = self._get_rand() as f64;
        (rand / 4294967296.0) * upper
    }

    fn random_range(self: &mut Self, lower: f64, upper: f64) -> f64 {
        let diff = (upper - lower).abs();
        let rand = self.random(1.0);
        lower.min(upper) + rand * diff
    }

    fn irandom(self: &mut Self, upper: i64) -> i64 {
        // upper = upper.floor() as i64;
        let signum = if upper < 0 { -1 } else { 1 };
        // self._get_rand_long(upper + signum) as u32 as f64
        self._get_rand_long(upper + signum)
    }

    fn irandom_range(self: &mut Self, lower: i64, upper: i64) -> i64 {
        // lower = lower.floor() as i64;
        // upper = upper.floor() as i64;
        let diff = (upper - lower).abs();
        let rand = self._get_rand_long(diff + 1);
        // (lower.min(upper) + rand) as u32 as f64
        lower.min(upper) + rand
    }

    // fn yyrandomrange(self: &mut Self, mut upper: f64) -> f64 {
    //     upper = upper.floor();
    //     let rand = self._get_rand() as f64;
    //     let v3 = upper.abs();
    //     let mut v1 = 0.0;
    //     if v3 != 0.0 {
    //         v1 = rand.div_euclid(v3);
    //     }
    //     rand - v1 * v3
    // }

    fn choose<T: Copy>(self: &mut Self, list: &[T]) -> Option<T> {
        if list.len() == 0 {
            return None;
        }
        let rand = self._get_rand_upper(list.len() as u32);
        // Top 10 most cursed type castings
        let floored = (rand as f32).floor() as u32 as usize;
        if floored < list.len() {
            Some(list[floored])
        } else {
            Some(list[list.len() - 1])
        }
    }

    // Called math_switch_random in RnS code
    fn mino_choose<T: Copy>(self: &mut Self, list: &[T]) -> Option<T> {
        if list.len() == 0 {
            return None;
        }
        Some(list[self.random(list.len() as f64).floor() as usize])
    }

    fn shuffle<T>(self: &mut Self, list: &mut [T]) {
        let len = list.len() as u32;
        for _ in 0..8 * list.len() {
            let index1 = self._get_rand_upper(len) as usize;
            let index2 = self._get_rand_upper(len) as usize;
            list.swap(index1, index2);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Shop {
    gems: [(usize, usize); 4],    // id, price
    potions: [(usize, usize); 3], // id, price
}

impl Shop {
    fn new() -> Self {
        Self {
            gems: [(0, 0); 4],
            potions: [(0, 0); 3],
        }
    }
}

#[derive(Debug, Clone)]
struct Chest {
    color: usize, // TODO: use enum instead
    items: Vec<usize>,
}

impl Chest {
    fn new(color: usize, items: Vec<usize>) -> Self {
        Self { color, items }
    }

    fn get_dummy() -> Self {
        Self {
            color: 0,
            items: vec![],
        }
    }
}

#[derive(Debug)]
struct Run {
    map_seed: u32,
    players: usize,
    high_difficulty: bool,
    rand: GMRand,
    hallseeds: [u32; 6],   // Idk why there's 6, copied from glass' GML code
    area_list: [usize; 5], // "id", homemade
    outskirts: [(usize, usize); 5], // "id", pattern
    pale_keep: [usize; 5], // "id"
    shops: [Shop; 4],
    chests: [Chest; 6],
}

impl Run {
    fn new(map_seed: u32, players: usize, high_difficulty: bool) -> Self {
        Self {
            map_seed,
            players,
            high_difficulty,
            rand: GMRand::new(),
            hallseeds: [0; 6],
            area_list: [0, 1, 2, 3, 4],
            outskirts: [(0, 0); 5],
            pale_keep: [0, 1, 2, 3, 4],
            shops: [Shop::new(); 4],
            chests: [
                Chest::get_dummy(),
                Chest::get_dummy(),
                Chest::get_dummy(),
                Chest::get_dummy(),
                Chest::get_dummy(),
                Chest::get_dummy(),
            ],
        }
    }

    fn get_all_list() -> [Vec<usize>; 24] {
        [
            vec![
                // Idk why it's here but have to keep it since it affects RNG
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
                65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
                86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104,
                105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
                121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136,
                137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
                153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
                169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
                185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
                201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216,
                217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232,
                233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248,
                249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264,
                265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280,
                281, 282, 283, 284, 285, 286, 487, 488,
            ],
            vec![], // Always empty(?), don't know if this one is necessary to keep
            vec![], // White chest
            vec![
                // Opal chest
                287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
                304, 305, 306, 307, 308, 309, 310,
            ],
            vec![
                // Sapphire chest
                311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327,
                328, 329, 330, 331, 332, 333, 334,
            ],
            vec![
                // Ruby chest
                335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351,
                352, 353, 354, 355, 356, 357, 358,
            ],
            vec![
                // Garnet chest
                359, 360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375,
                376, 377, 378, 379, 380, 381, 382,
            ],
            vec![
                // Emerald chest
                383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399,
                400, 401, 402, 403, 404, 405, 406,
            ],
            vec![407, 408, 409, 410, 411, 412, 413, 414], // os set, i.e. Opal/Sapphire set
            vec![415, 416, 417, 418, 419, 420, 421, 422], // or set
            vec![423, 424, 425, 426, 427, 428, 429, 430], // og set
            vec![431, 432, 433, 434, 435, 436, 437, 438], // oe set
            vec![439, 440, 441, 442, 443, 444, 445, 446], // sr set
            vec![447, 448, 449, 450, 451, 452, 453, 454], // sg set
            vec![455, 456, 457, 458, 459, 460, 461, 462], // se set
            vec![463, 464, 465, 466, 467, 468, 469, 470], // rg set
            vec![471, 472, 473, 474, 475, 476, 477, 478], // re set
            vec![479, 480, 481, 482, 483, 484, 485, 486], // ge set
            vec![489],                                    // Regen potion
            vec![
                // Other potions
                490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503,
            ],
            vec![504, 508, 512, 516, 520], // Primaries
            vec![505, 509, 513, 517, 521], // Secondaries
            vec![506, 510, 514, 518, 522], // Specials
            vec![507, 511, 515, 519, 523], // Defensives
        ]
    }

    fn predict_seed(self: &mut Self) {
        let r = &mut self.rand;
        let mut all_list = Run::get_all_list();

        // Hallseeds
        r.set_seed(self.map_seed);
        self.hallseeds[0] = r.irandom_range(0, 4294967295) as u32;
        r.shuffle(&mut self.area_list);
        self.hallseeds[1] = r.irandom(4294967295) as u32;
        self.hallseeds[2] = r.irandom(4294967295) as u32;
        self.hallseeds[3] = r.irandom(4294967295) as u32;
        self.hallseeds[4] = r.irandom(4294967295) as u32;
        self.hallseeds[5] = r.irandom(4294967295) as u32;

        // Items
        r.set_seed(self.map_seed + 5);
        // Algo:
        // Shuffle all
        // Add sets to their respective colored chests
        // Shuffle all
        // Move 8 items from colored chests to white chest
        // Shuffle all
        // Shuffle chest list twice
        // Get 6 chests and their items

        for list in all_list.iter_mut() {
            r.shuffle(list);
        }

        // all_list up to and including chest lists, and all_list from sets and onwards
        let (chests, sets) = all_list.split_at_mut(8);
        chests[3].extend(&sets[0][0..4]);
        chests[3].extend(&sets[1][0..4]);
        chests[3].extend(&sets[2][0..4]);
        chests[3].extend(&sets[3][0..4]);

        chests[4].extend(&sets[0][4..8]);
        chests[4].extend(&sets[4][0..4]);
        chests[4].extend(&sets[5][0..4]);
        chests[4].extend(&sets[6][0..4]);

        chests[5].extend(&sets[1][4..8]);
        chests[5].extend(&sets[4][4..8]);
        chests[5].extend(&sets[7][0..4]);
        chests[5].extend(&sets[8][0..4]);

        chests[6].extend(&sets[2][4..8]);
        chests[6].extend(&sets[5][4..8]);
        chests[6].extend(&sets[7][4..8]);
        chests[6].extend(&sets[9][0..4]);

        chests[7].extend(&sets[3][4..8]);
        chests[7].extend(&sets[6][4..8]);
        chests[7].extend(&sets[8][4..8]);
        chests[7].extend(&sets[9][4..8]);

        for list in all_list.iter_mut() {
            r.shuffle(list);
        }

        // all_list up to and incl white chest, and the rest
        let (white_chest, other_chests) = all_list.split_at_mut(3);
        for i in 0..5 {
            white_chest[2].extend(other_chests[i].drain(0..8));
        }

        for list in all_list.iter_mut() {
            r.shuffle(list);
        }

        let mut chest_types = [2, 2, 2, 3, 4, 5, 6, 7];
        // Yes, we do shuffle twice, not a typo
        r.shuffle(&mut chest_types);
        r.shuffle(&mut chest_types);

        let mut white_chest_iter = all_list[2].iter();
        for (chest_index, &chest_type) in chest_types[0..6].iter().enumerate() {
            let mut will_take = 5;
            if chest_index >= 2 {
                will_take = match self.players {
                    1 => 3,
                    2 => 4,
                    3 => 4,
                    _ => 5,
                };
            }
            let item_iter = if chest_type == 2 {
                &mut white_chest_iter
            } else {
                &mut all_list[chest_type].iter()
            };
            let mut item_list = vec![];
            for _ in 0..will_take {
                let mut item_id = *item_iter.next().unwrap();
                while chest_index >= 4 && item_id == 357
                    || chest_index >= 5 && (item_id == 387 || item_id == 380 || item_id == 383)
                {
                    // Skip over banned items in area 3,4
                    item_id = *item_iter.next().unwrap()
                }
                item_list.push(item_id);
            }
            self.chests[chest_index] = Chest::new(chest_type, item_list);
        }

        // Outskirts
        r.set_seed(self.hallseeds[0]);
        for fight in 0..5 {
            let pattern = r.mino_choose(&[0, 1]).unwrap();
            self.outskirts[fight] = (fight, pattern);
        }
        r.shuffle(&mut self.outskirts);

        // Areas
        for area_index in 1..=4 {
            r.set_seed(self.hallseeds[area_index]);
            if area_index == 4 {
                // TODO: Check that this is correct
                r.shuffle(&mut self.pale_keep);
            }
            let rand = r.random(2147483647.0).floor() as u32;
            r.set_seed(rand);

            let shop = &mut self.shops[area_index - 1];

            let mut potions = all_list[19].clone();
            r.shuffle(&mut potions);
            if area_index == 1 && self.high_difficulty {
                potions[0] = 489;
            }
            let mut potion_index = 0;
            for i in 0..3 {
                if area_index >= 3 && potions[potion_index] == 498 {
                    // Skip golden potion for area 3,4
                    potion_index += 1;
                }
                let potion_id = potions[potion_index];
                let price = if i == 0 && area_index == 1 && self.high_difficulty {
                    8 // Regen pot always costs 8
                } else {
                    r.irandom_range(7, 10) as usize
                };
                shop.potions[i] = (potion_id, price);
                potion_index += 1;
            }

            let mut gems = [
                all_list[20].clone(),
                all_list[21].clone(),
                all_list[22].clone(),
                all_list[23].clone(),
            ];
            for i in 0..4 {
                r.shuffle(&mut gems[i]);
                let gem_id = gems[i][0];
                let price = r.irandom_range(23, 27) as usize;
                shop.gems[i] = (gem_id, price);
            }
        }
    }

    fn get_csv_line(self: Self) -> String {
        let mut out = String::new();

        let area_string = self
            .area_list
            .iter()
            .map(|&area_index| AREA_NAMES[area_index])
            .collect::<Vec<_>>()
            .join(",");
        let outskirt_string = self
            .outskirts
            .iter()
            .take(3) // Only output 3 fights
            .map(|&(fight_index, pattern_index)| OUTSKIRT_NAMES[fight_index][pattern_index])
            .collect::<Vec<_>>()
            .join(",");
        let pale_keep_string = self
            .pale_keep
            .iter()
            .take(3) // Only output 3 fights
            .map(|&index| PALE_KEEP_NAMES[index])
            .collect::<Vec<_>>()
            .join(",");
        let item_string = self
            .chests
            .iter()
            .map(|chest| &chest.items)
            .map(|items| {
                items
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join(",");

        out.push_str(&self.map_seed.to_string());
        out.push(',');
        out.push_str(&area_string);
        out.push(',');
        out.push_str(&outskirt_string);
        out.push(',');
        out.push_str(&pale_keep_string);
        out.push(',');
        out.push_str(&item_string);
        out.push(',');

        for shop in self.shops {
            for (id, _) in shop.potions {
                out.push_str(&id.to_string());
                out.push(',');
            }
            for (_, price) in shop.potions {
                out.push_str(&price.to_string());
                out.push(',');
            }
            for (id, price) in shop.gems {
                out.push_str(&id.to_string());
                out.push(',');
                out.push_str(&price.to_string());
                out.push(',');
            }
        }

        out
    }
}

impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hallseed_string = self
            .hallseeds
            .iter()
            .map(|hallseed| hallseed.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let area_string = self
            .area_list
            .iter()
            .map(|&area_index| AREA_NAMES[area_index])
            .collect::<Vec<_>>()
            .join(",");
        let outskirt_string = self
            .outskirts
            .iter()
            .map(|&(fight_index, pattern_index)| OUTSKIRT_NAMES[fight_index][pattern_index])
            .collect::<Vec<_>>()
            .join(",");
        let pale_keep_string = self
            .pale_keep
            .iter()
            .map(|&index| PALE_KEEP_NAMES[index])
            .collect::<Vec<_>>()
            .join(",");
        let difficulty_string = if self.high_difficulty {
            "Hard/Lunar"
        } else {
            "Cute/Normal"
        };

        writeln!(f, "Seed: {}", self.map_seed)?;
        writeln!(f, "Players: {}", self.players)?;
        writeln!(f, "Difficulty: {}", difficulty_string)?;
        writeln!(f, "Hallseeds: [{}]", hallseed_string)?;
        writeln!(f, "Areas: [{}]", area_string)?;
        writeln!(f, "Outskirts: [{}]", outskirt_string)?;
        writeln!(f, "Pale Keep: [{}]", pale_keep_string)?;

        writeln!(f)?;
        // writeln!(f, "Shops:")?;
        for (i, shop) in self.shops.into_iter().enumerate() {
            let gem_names = shop
                .gems
                .iter()
                .map(|&(id, _)| OBJECT_NAMES[id])
                .collect::<Vec<_>>()
                .join(", ");
            let gem_prices = shop
                .gems
                .iter()
                .map(|&(_, price)| price.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let potion_names = shop
                .potions
                .iter()
                .map(|&(id, _)| OBJECT_NAMES[id])
                .collect::<Vec<_>>()
                .join(", ");
            let potion_prices = shop
                .potions
                .iter()
                .map(|&(_, price)| price.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "Shop {}:", i)?;
            writeln!(f, "  Gems: {}", gem_names)?;
            writeln!(f, "  Prices: {}", gem_prices)?;
            writeln!(f, "  Potions: {}", potion_names)?;
            writeln!(f, "  Prices: {}", potion_prices)?;
        }
        writeln!(f)?;
        // writeln!(f, "Items:")?;
        for (i, chest) in self.chests.iter().enumerate() {
            let item_names = chest
                .items
                .iter()
                .map(|&id| OBJECT_NAMES[id])
                .collect::<Vec<_>>()
                .join(", ");
            let color = CHEST_NAMES[chest.color - 2];

            writeln!(f, "Chest {} - {}:", i, color)?;
            writeln!(f, "  Items: {}", item_names)?;
        }
        Ok(())
    }
}

// Get enough state to tell completely whether two seeds will be equal
fn get_short_state(seed: u32) -> (u32, u32) {
    (
        (seed * 0x343fd + 0x269ec3) >> 16,
        ((seed + 5) * 0x343fd + 0x269ec3) >> 16,
    )
}

fn generate_csv() {
    // Only here for optimization, can be raised if RNG changes
    let unique_seeds = 2usize.pow(17);
    let mut states: Vec<(u32, u32)> = Vec::with_capacity(unique_seeds);
    let mut lines: Vec<String> = Vec::with_capacity(unique_seeds);
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

        // Run prediction
        let mut run = Run::new(seed, 4, true);
        run.predict_seed();
        lines.push(run.get_csv_line());

        if lines.len() % 10000 == 0 {
            println!("{} unique seeds found so far", states.len());
        }
    }
    let mut file = File::create("unique_seeds.csv")
        .expect("Expected to be able to create file unique_seeds.csv");
    file.write_all(lines.join("\n").as_bytes())
        .expect("Expected to be able to write to file unique_seeds.csv");
}

fn main() {
    if env::args().len() == 1 {
        println!("Generating csv file");
        generate_csv();
        return;
    }
    let mut seed = 1585;
    let mut players = 4;
    let mut high_difficulty = true;
    for (i, arg) in env::args().enumerate() {
        match i {
            1 => seed = arg.parse().expect("Expected arg to be unsigned integer"),
            2 => players = arg.parse().expect("Expected arg to be unsigned integer"),
            3 => high_difficulty = arg.parse().expect("Expected arg to be bool"),
            _ => {}
        }
    }
    let mut run = Run::new(seed, players, high_difficulty);
    run.predict_seed();
    println!("{}", run);
    println!("{}", run.get_csv_line())
}
