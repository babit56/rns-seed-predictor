pub mod names;
pub mod rand;
pub mod types;

use std::{array, fmt};

use rand::GMRand;
use types::{Chest, Gem, Potion, Shop, Unlocks};

#[derive(Debug, Clone)]
pub struct Run {
    map_seed: u32,
    players: u8,
    high_difficulty: bool,
    rand: GMRand,
    hallseeds: [u32; 6],   // Idk why there's 6, copied from glass' GML code
    area_list: [usize; 5], // "id", homemade
    outskirts: [(usize, usize); 5], // "id", pattern
    pale_keep: [usize; 5], // "id"
    shops: [Option<Shop>; 4],
    chests: [Option<Chest>; 6],
    unlocks: Unlocks,
}

impl Run {
    pub fn new(map_seed: u32, players: u8, high_difficulty: bool, unlocks: Unlocks) -> Self {
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
            unlocks,
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

    #[rustfmt::skip]
    fn fill_color_chests(self: &Self, all_list: &mut [Vec<usize>; 24]) {
        // all_list up to and including chest lists, and all_list from sets and onwards
        let (chests, sets) = all_list.split_at_mut(8);

        if self.unlocks.darkbite    { chests[3].extend(&sets[0][0..4]); }
        if self.unlocks.timegem     { chests[3].extend(&sets[1][0..4]); }
        if self.unlocks.youkai      { chests[3].extend(&sets[2][0..4]); }
        if self.unlocks.haunted     { chests[3].extend(&sets[3][0..4]); }

        if self.unlocks.darkbite    { chests[4].extend(&sets[0][4..8]); }
        if self.unlocks.gladiator   { chests[4].extend(&sets[4][0..4]); }
        if self.unlocks.sparkblade  { chests[4].extend(&sets[5][0..4]); }
        if self.unlocks.swiftflight { chests[4].extend(&sets[6][0..4]); }

        if self.unlocks.timegem     { chests[5].extend(&sets[1][4..8]); }
        if self.unlocks.gladiator   { chests[5].extend(&sets[4][4..8]); }
        if self.unlocks.sacredflame { chests[5].extend(&sets[7][0..4]); }
        if self.unlocks.ruins       { chests[5].extend(&sets[8][0..4]); }

        if self.unlocks.youkai      { chests[6].extend(&sets[2][4..8]); }
        if self.unlocks.sparkblade  { chests[6].extend(&sets[5][4..8]); }
        if self.unlocks.sacredflame { chests[6].extend(&sets[7][4..8]); }
        if self.unlocks.lakeshrine  { chests[6].extend(&sets[9][0..4]); }

        if self.unlocks.haunted     { chests[7].extend(&sets[3][4..8]); }
        if self.unlocks.swiftflight { chests[7].extend(&sets[6][4..8]); }
        if self.unlocks.ruins       { chests[7].extend(&sets[8][4..8]); }
        if self.unlocks.lakeshrine  { chests[7].extend(&sets[9][4..8]); }
    }

    pub fn predict_seed(self: &mut Self) {
        let mut all_list = Run::get_all_list();

        // Hallseeds
        self.rand.set_seed(self.map_seed);
        self.hallseeds[0] = self.rand.irandom_range(0, 4294967295) as u32;
        self.rand.shuffle(&mut self.area_list);
        self.hallseeds[1] = self.rand.irandom(4294967295) as u32;
        self.hallseeds[2] = self.rand.irandom(4294967295) as u32;
        self.hallseeds[3] = self.rand.irandom(4294967295) as u32;
        self.hallseeds[4] = self.rand.irandom(4294967295) as u32;
        self.hallseeds[5] = self.rand.irandom(4294967295) as u32;

        // Items
        self.rand.set_seed(self.map_seed + 5);

        for list in all_list.iter_mut() {
            self.rand.shuffle(list);
        }

        self.fill_color_chests(&mut all_list);

        for list in all_list.iter_mut() {
            self.rand.shuffle(list);
        }

        // all_list up to and incl white chest, and the rest
        let (white_chest, other_chests) = all_list.split_at_mut(3);
        for i in 0..5 {
            white_chest[2].extend(other_chests[i].drain(0..8));
        }

        for list in all_list.iter_mut() {
            self.rand.shuffle(list);
        }

        let mut chest_types = [2, 2, 2, 3, 4, 5, 6, 7];
        // Yes, we do shuffle twice, not a typo
        self.rand.shuffle(&mut chest_types);
        self.rand.shuffle(&mut chest_types);

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
        self.rand.set_seed(self.hallseeds[0]);
        for fight in 0..5 {
            let pattern = self.rand.mino_choose(&[0, 1]).unwrap();
            self.outskirts[fight] = (fight, pattern);
        }
        self.rand.shuffle(&mut self.outskirts);

        // Areas
        for area_index in 1..=4 {
            self.rand.set_seed(self.hallseeds[area_index]);
            if area_index == 4 {
                self.rand.shuffle(&mut self.pale_keep);
            }
            let rand = self.rand.random(2147483647.0).floor() as u32;
            self.rand.set_seed(rand);

            // Potions
            let mut potion_list = all_list[19].clone();
            self.rand.shuffle(&mut potion_list);
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
                        self.rand.irandom_range(7, 10) as usize
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
                self.rand.shuffle(&mut gem_lists[i]);
                let gem_id = gem_lists[i][0];
                let price = self.rand.irandom_range(23, 27) as usize;
                Gem::from_id_price(gem_id, price).unwrap()
            });

            self.shops[area_index - 1] = Some(Shop::new(gems, potions));
        }
    }

    // fn generate_shop() -> Shop {}

    pub fn get_csv_line(self: &Self) -> String {
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
            .map(|&(fight_index, pattern_index)| {
                names::get_outskirt_name(fight_index, pattern_index).unwrap()
            })
            .take(3) // Only output 3 fights
            .collect::<Vec<_>>()
            .join(",");
        let pale_keep_string = self
            .pale_keep
            .iter()
            .map(|&index| names::get_pale_keep_name(index).unwrap())
            .take(3) // Only output 3 fights
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
                out.push_str(&gem.gem_id.to_string());
                out.push(',');
                out.push_str(&gem.price.to_string());
                out.push(',');
            }
        }
        out.pop(); // Remove last comma

        out
    }

    pub fn get_short_line(self: &Self) -> String {
        let mut out = String::new();

        let area_string = self
            .area_list
            .iter()
            .map(|&area_index| area_index.to_string())
            .take(3) // Only output 3 areas
            .collect::<Vec<_>>()
            .join(",");
        let outskirt_string = self
            .outskirts
            .iter()
            .map(|&(fight_index, pattern_index)| {
                // Write outskirt id where the ten's digit is the fight (1-5) and one's digit is the pattern (1-2)
                ((fight_index + 1) * 10 + pattern_index + 1).to_string()
            })
            .take(3) // Only output 3 fights
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
                out.push_str(&gem.gem_id.to_string());
                out.push(',');
                out.push_str(&gem.price.to_string());
                out.push(',');
            }
        }
        out.pop(); // Remove last comma

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
            .join(", ");
        let area_string = self
            .area_list
            .iter()
            .map(|&area_index| names::get_area_name_pretty(area_index).unwrap())
            .collect::<Vec<_>>()
            .join(", ");
        let outskirt_string = self
            .outskirts
            .iter()
            .map(|&(fight_index, pattern_index)| {
                names::get_outskirt_name(fight_index, pattern_index).unwrap()
            })
            .take(3)
            .collect::<Vec<_>>()
            .join(", ");
        let pale_keep_string = self
            .pale_keep
            .iter()
            .map(|&index| names::get_pale_keep_name_pretty(index).unwrap())
            .take(3)
            .collect::<Vec<_>>()
            .join(", ");
        let difficulty_string = if self.high_difficulty {
            "Hard/Lunar"
        } else {
            "Cute/Normal"
        };
        let get_shop_things = |shop: Shop| {
            let gem_names = shop
                .gems
                .iter()
                .map(|gem| gem.gem_type.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let gem_prices = shop
                .gems
                .iter()
                .map(|gem| gem.price.to_string() + "g")
                .collect::<Vec<_>>()
                .join(", ");
            let potion_names = shop
                .potions
                .iter()
                .map(|potion| names::get_item_name(potion.potion_id).unwrap())
                .collect::<Vec<_>>()
                .join(", ");
            let potion_prices = shop
                .potions
                .iter()
                .map(|potion| potion.price.to_string() + "g")
                .collect::<Vec<_>>()
                .join(", ");
            (gem_names, gem_prices, potion_names, potion_prices)
        };

        writeln!(f, "Seed: {}", self.map_seed)?;
        writeln!(f, "Players: {}", self.players)?;
        writeln!(f, "Difficulty: {}", difficulty_string)?;
        writeln!(f, "Hallseeds: [{}]", hallseed_string)?;
        writeln!(f, "Areas: [{}]", area_string)?;
        writeln!(f, "Outskirts enemies: [{}]", outskirt_string)?;
        writeln!(f, "Pale Keep enemies: [{}]", pale_keep_string)?;

        writeln!(f)?;
        // writeln!(f, "Shops:")?;
        for (i, shop) in self.shops.into_iter().enumerate() {
            let (gem_names, gem_prices, potion_names, potion_prices) =
                get_shop_things(shop.unwrap());
            writeln!(f, "Shop {}:", i + 1)?;
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
            writeln!(f, "Chest {} - {}:", i + 1, color)?;
            writeln!(f, "  {}", item_names)?;
        }
        Ok(())
    }
}
