use std::fmt::Display;

#[derive(Debug)]
struct GMRand {
    seed: u32,
    state: [u32; 16],
    index: usize,
    random_poly: u32,
}

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
            temp = (temp * 0x343fd + 0x269ec3) >> 16;
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

#[derive(Debug)]
struct Run {
    map_seed: u32,
    rand: GMRand,
    hallseeds: [u32; 6], // Idk why there's 6, copied from glass' GML code
    area_list: [usize; 5],
    outskirts: [(usize, usize); 5],
    // ...
}

impl Run {
    fn new(map_seed: u32) -> Self {
        Self {
            map_seed,
            rand: GMRand::new(),
            hallseeds: [0; 6],
            area_list: [0, 1, 2, 3, 4],
            outskirts: [(0, 0); 5],
        }
    }

    fn predict_seed(self: &mut Self) {
        let r = &mut self.rand;

        // Hallseeds
        r.set_seed(self.map_seed);
        self.hallseeds[0] = r.irandom_range(0, 4294967295) as u32;
        r.shuffle(&mut self.area_list);
        self.hallseeds[1] = r.irandom(4294967295) as u32;
        self.hallseeds[2] = r.irandom(4294967295) as u32;
        self.hallseeds[3] = r.irandom(4294967295) as u32;
        self.hallseeds[4] = r.irandom(4294967295) as u32;
        self.hallseeds[5] = r.irandom(4294967295) as u32;

        // Outskirts
        r.set_seed(self.hallseeds[0]);
        for fight in 0..5 {
            // Kinda hacky, probably better ways to do this
            let pattern = r.mino_choose(&[0, 1]).unwrap();
            self.outskirts[fight] = (fight, pattern);
        }
        r.shuffle(&mut self.outskirts);

        // Areas

        // Items
    }
}

impl Display for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area_list = [
            "hw_nest",
            "hw_arsenal",
            "hw_lighthouse",
            "hw_streets",
            "hw_lakeside",
        ];
        let outskirt_fights = [
            ["enc_bird_sophomore1", "enc_bird_sophomore2"],
            ["enc_frog_tinkerer1", "enc_frog_tinkerer2"],
            ["enc_dragon_granite1", "enc_dragon_granite2"],
            ["enc_wolf_blackear1", "enc_wolf_blackear2"],
            ["enc_mouse_cadet1", "enc_mouse_cadet2"],
        ];

        let hallseed_string = self
            .hallseeds
            .iter()
            .map(|hallseed| hallseed.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let area_string = self
            .area_list
            .iter()
            .map(|&area_index| area_list[area_index])
            .collect::<Vec<_>>()
            .join(",");
        let outskirt_string = self
            .outskirts
            .iter()
            .map(|&(fight_index, pattern_index)| outskirt_fights[fight_index][pattern_index])
            .collect::<Vec<_>>()
            .join(",");

        writeln!(f, "Seed: {}", self.map_seed)?;
        writeln!(f, "Hallseeds: [{}]", hallseed_string)?;
        writeln!(f, "Areas: [{}]", area_string)?;
        writeln!(f, "Outskirts: [{}]", outskirt_string)?;
        Ok(())
    }
}

fn main() {
    let mut run = Run::new(1585);
    run.predict_seed();
    println!("{}", run);
}
