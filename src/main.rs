use std::{array, env, fmt, fs::File, io::Write};

use rns_seed_predictor::{names, Chest, Gem, Potion, Shop};

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
    shops: [Option<Shop>; 4],
    chests: [Option<Chest>; 6],
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
            shops: [None; 4],
            chests: [None, None, None, None, None, None],
        }
    }

    fn get_all_list() -> [Vec<usize>; 24] {
        // See NOTES.md for information the contents of each list
        [
            vec![
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
            vec![],
            vec![],
            vec![
                287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302,
                303, 304, 305, 306, 307, 308, 309, 310,
            ],
            vec![
                311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326,
                327, 328, 329, 330, 331, 332, 333, 334,
            ],
            vec![
                335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 350,
                351, 352, 353, 354, 355, 356, 357, 358,
            ],
            vec![
                359, 360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374,
                375, 376, 377, 378, 379, 380, 381, 382,
            ],
            vec![
                383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398,
                399, 400, 401, 402, 403, 404, 405, 406,
            ],
            vec![407, 408, 409, 410, 411, 412, 413, 414],
            vec![415, 416, 417, 418, 419, 420, 421, 422],
            vec![423, 424, 425, 426, 427, 428, 429, 430],
            vec![431, 432, 433, 434, 435, 436, 437, 438],
            vec![439, 440, 441, 442, 443, 444, 445, 446],
            vec![447, 448, 449, 450, 451, 452, 453, 454],
            vec![455, 456, 457, 458, 459, 460, 461, 462],
            vec![463, 464, 465, 466, 467, 468, 469, 470],
            vec![471, 472, 473, 474, 475, 476, 477, 478],
            vec![479, 480, 481, 482, 483, 484, 485, 486],
            vec![489],
            vec![
                490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503,
            ],
            vec![504, 508, 512, 516, 520],
            vec![505, 509, 513, 517, 521],
            vec![506, 510, 514, 518, 522],
            vec![507, 511, 515, 519, 523],
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
            self.chests[chest_index] = Chest::from_id(chest_type, item_list)
                .inspect_err(|e| eprintln!("Failed to create Chest: {}", e))
                .ok();
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
                r.shuffle(&mut self.pale_keep);
            }
            let rand = r.random(2147483647.0).floor() as u32;
            r.set_seed(rand);

            // Potions
            let mut potion_list = all_list[19].clone();
            r.shuffle(&mut potion_list);
            if area_index == 1 && self.high_difficulty {
                potion_list[0] = 489;
            }
            let mut potion_iter = potion_list
                .into_iter()
                .filter(|&potion_id| !(area_index >= 3 && potion_id == 498))
                .enumerate()
                .map(|(i, potion_id)| {
                    let price = if i == 0 && area_index == 1 && self.high_difficulty {
                        8 // Regen pot always costs 8
                    } else {
                        r.irandom_range(7, 10) as usize
                    };
                    (potion_id, price)
                })
                .map(|(potion_id, price)| Potion::from_id_price(potion_id, price).unwrap())
                .take(3);
            let potions = array::from_fn(|_| potion_iter.next().unwrap());

            // Gems
            let mut gem_lists = [
                all_list[20].clone(),
                all_list[21].clone(),
                all_list[22].clone(),
                all_list[23].clone(),
            ];
            let gems = array::from_fn(|i| {
                r.shuffle(&mut gem_lists[i]);
                let gem_id = gem_lists[i][0];
                let price = r.irandom_range(23, 27) as usize;
                Gem::from_id_price(gem_id, price).unwrap()
            });

            self.shops[area_index - 1] = Some(Shop::new(gems, potions));
        }
    }

    // fn generate_shop() -> Shop {}

    fn get_csv_line(self: Self) -> String {
        let mut out = String::new();

        let area_string = self
            .area_list
            .iter()
            .map(|&area_index| names::get_area_name(area_index).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        let outskirt_string = self
            .outskirts
            .iter()
            .take(3) // Only output 3 fights
            .map(|&(fight_index, pattern_index)| {
                names::get_outskirt_name(fight_index, pattern_index).unwrap()
            })
            .collect::<Vec<_>>()
            .join(",");
        let pale_keep_string = self
            .pale_keep
            .iter()
            .take(3) // Only output 3 fights
            .map(|&index| names::get_pale_keep_name(index).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        let item_string = self
            .chests
            .iter()
            .map(|chest| {
                chest
                    .as_ref()
                    .unwrap()
                    .items
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
            for potion in shop.unwrap().potions {
                out.push_str(&potion.potion_id.to_string());
                out.push(',');
            }
            for potion in shop.unwrap().potions {
                out.push_str(&potion.price.to_string());
                out.push(',');
            }
            for gem in shop.unwrap().gems {
                out.push_str(&(gem.gem_type as usize).to_string());
                out.push(',');
                out.push_str(&gem.price.to_string());
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
            .map(|&area_index| names::get_area_name(area_index).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        let outskirt_string = self
            .outskirts
            .iter()
            .map(|&(fight_index, pattern_index)| {
                names::get_outskirt_name(fight_index, pattern_index).unwrap()
            })
            .collect::<Vec<_>>()
            .join(",");
        let pale_keep_string = self
            .pale_keep
            .iter()
            .map(|&index| names::get_pale_keep_name(index).unwrap())
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
                .unwrap()
                .gems
                .iter()
                .map(|gem| gem.gem_type.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let gem_prices = shop
                .unwrap()
                .gems
                .iter()
                .map(|gem| gem.price.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let potion_names = shop
                .unwrap()
                .potions
                .iter()
                .map(|potion| names::get_item_name(potion.potion_id).unwrap())
                .collect::<Vec<_>>()
                .join(", ");
            let potion_prices = shop
                .unwrap()
                .potions
                .iter()
                .map(|potion| potion.price.to_string())
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
                .as_ref()
                .unwrap()
                .items
                .iter()
                .map(|&id| names::get_item_name(id).unwrap())
                .collect::<Vec<_>>()
                .join(", ");

            let color = chest.as_ref().unwrap().color;
            writeln!(f, "Chest {} - {}:", i, color)?;
            writeln!(f, "  Items: {}", item_names)?;
        }
        Ok(())
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
