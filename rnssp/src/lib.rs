pub mod hallwaygen;
pub mod names;
pub mod types;

use std::{array, fmt};

use gamemaker_rand::{GMRand, Real, rng};
use types::{
    Area::{self, *},
    Chest, Encounter, Gem, Potion, Shop, StartingArea, Unlocks,
};

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone)]
pub struct Run {
    pub map_seed: u32,
    pub players: u8,
    pub high_difficulty: bool,
    pub starting_area: StartingArea,
    #[cfg_attr(feature = "serde_support", serde(skip, default = "rng"))]
    rand: gamemaker_rand::WELL512a,
    pub hallseeds: [u32; 6], // ouskirts + 3 areas + end area + endboss
    pub area_list: [Area; 5],
    pub encounters: [Encounter; 19],
    pub shops: [Option<Shop>; 4],
    pub chests: [Option<Chest>; 6],
    pub unlocks: Unlocks,
}

impl Run {
    pub fn new(
        map_seed: u32,
        players: u8,
        high_difficulty: bool,
        starting_area: StartingArea,
        unlocks: Unlocks,
    ) -> Self {
        Self {
            map_seed,
            players,
            high_difficulty,
            starting_area,
            rand: rng(),
            hallseeds: [0; 6],
            area_list: [Outskirts; 5],
            encounters: [Encounter::TrainingNothing; 19],
            shops: [None; 4],
            chests: [None, None, None, None, None, None],
            unlocks,
        }
    }

    fn get_all_items_list() -> [Vec<usize>; 34] {
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
                281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296,
                297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312,
                313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328,
                329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
                345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360,
                361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376,
                377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392,
                393, 394, 395, 396, 397, 398, 679, 680, 696,
            ],
            vec![],
            vec![],
            vec![
                399, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414,
                415, 416, 417, 418, 419, 420, 421, 422,
            ],
            vec![
                423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438,
                439, 440, 441, 442, 443, 444, 445, 446,
            ],
            vec![
                447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462,
                463, 464, 465, 466, 467, 468, 469, 470,
            ],
            vec![
                471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486,
                487, 488, 489, 490, 491, 492, 493, 494,
            ],
            vec![
                495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510,
                511, 512, 513, 514, 515, 516, 517, 518,
            ],
            vec![519, 520, 521, 522, 523, 524, 525, 526],
            vec![527, 528, 529, 530, 531, 532, 533, 534],
            vec![535, 536, 537, 538, 539, 540, 541, 542],
            vec![543, 544, 545, 546, 547, 548, 549, 550],
            vec![551, 552, 553, 554, 555, 556, 557, 558],
            vec![559, 560, 561, 562, 563, 564, 565, 566],
            vec![567, 568, 569, 570, 571, 572, 573, 574],
            vec![575, 576, 577, 578, 579, 580, 581, 582],
            vec![583, 584, 585, 586, 587, 588, 589, 590],
            vec![591, 592, 593, 594, 595, 596, 597, 598],
            vec![599, 600, 601, 602, 603, 604, 605, 606],
            vec![607, 608, 609, 610, 611, 612, 613, 614],
            vec![615, 616, 617, 618, 619, 620, 621, 622],
            vec![623, 624, 625, 626, 627, 628, 629, 630],
            vec![631, 632, 633, 634, 635, 636, 637, 638],
            vec![639, 640, 641, 642, 643, 644, 645, 646],
            vec![647, 648, 649, 650, 651, 652, 653, 654],
            vec![655, 656, 657, 658, 659, 660, 661, 662],
            vec![663, 664, 665, 666, 667, 668, 669, 670],
            vec![671, 672, 673, 674, 675, 676, 677, 678],
            vec![681],
            vec![
                682, 683, 684, 685, 686, 687, 688, 689, 690, 691, 692, 693, 694, 695,
            ],
            vec![697, 701, 705, 709, 713],
            vec![698, 702, 706, 710, 714],
            vec![699, 703, 707, 711, 715],
            vec![700, 704, 708, 712, 716],
        ]
    }

    /// See `scr_hallwayprogress_choose_halls`
    fn fill_area_list(self: &mut Self) {
        let seed = match self.starting_area {
            StartingArea::RandomKingdom => self.map_seed + 1,
            StartingArea::RandomExtra => self.map_seed + 2,
            StartingArea::TrueRandom => self.map_seed + 3,
            StartingArea::ChaoticRandom => self.map_seed + 4,
            _ => self.map_seed,
        };
        self.rand.set_seed(seed.into());

        // Outskirts/geode
        let intro_area = if self.starting_area.is_kingdom() {
            Outskirts
        } else if self.starting_area.is_extra() {
            Geode
        } else if self.starting_area.is_very_random() {
            self.math_random_switch(&[Outskirts, Geode]).unwrap()
        } else {
            panic!(
                "Starting area had unexpected value {:?}",
                self.starting_area
            );
        };

        // Areas
        let mut possible_areas = if self.starting_area.is_kingdom() {
            vec![Nest, Arsenal, Lighthouse, Streets, Lakeside]
        } else if self.starting_area.is_extra() {
            vec![Sanct, Depths, Aurum]
        } else if self.starting_area.is_very_random() {
            vec![
                Nest, Arsenal, Lighthouse, Streets, Lakeside, Sanct, Depths, Aurum,
            ]
        } else {
            panic!(
                "Starting area had unexpected value {:?}",
                self.starting_area
            );
        };

        let mut areas: [Area; 3] = [Outskirts; 3];
        if self.starting_area.is_random() {
            self.hallseeds[0] = self.rand.irandom(0xFFFFFFFFu32.into()).into();
            self.rand.ds_list_shuffle(&mut possible_areas);
            areas.copy_from_slice(&possible_areas[0..3]);
        } else {
            self.rand.ds_list_shuffle(&mut possible_areas);
            self.hallseeds[0] = self.rand.irandom(0xFFFFFFFFu32.into()).into();
            areas = [
                self.starting_area
                    .try_into()
                    .expect("Expected starting area to be non-random"),
                possible_areas[0],
                possible_areas[1],
            ]
        }

        // Areas
        self.hallseeds[1] = self.rand.irandom(0xFFFFFFFFu32.into()).into();
        self.hallseeds[2] = self.rand.irandom(0xFFFFFFFFu32.into()).into();
        self.hallseeds[3] = self.rand.irandom(0xFFFFFFFFu32.into()).into();

        // End area/boss area
        let end_area = if self.starting_area.is_kingdom() {
            Keep
        } else if self.starting_area.is_extra() {
            Darkhall
        } else if self.starting_area.is_very_random() {
            if self.math_coinflip() { Keep } else { Darkhall }
        } else {
            panic!(
                "Starting area had unexpected value {:?}",
                self.starting_area
            );
        };
        self.hallseeds[4] = self.rand.irandom(0xFFFFFFFFu32.into()).into();
        self.hallseeds[5] = self.rand.irandom(0xFFFFFFFFu32.into()).into();

        self.area_list = [intro_area, areas[0], areas[1], areas[2], end_area];
    }

    /// See `scr_hallwayprogress_add_unlocks`
    fn fill_color_chests(self: &mut Self, all_list: &mut [Vec<usize>; 34]) {
        // all_list up to and including chest lists, and all_list from sets and onwards
        let (chests, sets) = all_list.split_at_mut(8);

        if self.unlocks.darkbite {
            chests[3].extend(&sets[0][0..4]);
            chests[4].extend(&sets[0][4..8]);
        }
        if self.unlocks.timegem {
            chests[3].extend(&sets[1][0..4]);
            chests[5].extend(&sets[1][4..8]);
        }
        if self.unlocks.youkai {
            chests[3].extend(&sets[2][0..4]);
            chests[6].extend(&sets[2][4..8]);
        }
        if self.unlocks.haunted {
            chests[3].extend(&sets[3][0..4]);
            chests[7].extend(&sets[3][4..8]);
        }
        if self.unlocks.gladiator {
            chests[4].extend(&sets[4][0..4]);
            chests[5].extend(&sets[4][4..8]);
        }
        if self.unlocks.sparkblade {
            chests[4].extend(&sets[5][0..4]);
            chests[6].extend(&sets[5][4..8]);
        }
        if self.unlocks.swiftflight {
            chests[4].extend(&sets[6][0..4]);
            chests[7].extend(&sets[6][4..8]);
        }
        if self.unlocks.sacredflame {
            chests[5].extend(&sets[7][0..4]);
            chests[6].extend(&sets[7][4..8]);
        }
        if self.unlocks.ruins {
            chests[5].extend(&sets[8][0..4]);
            chests[7].extend(&sets[8][4..8]);
        }
        if self.unlocks.lakeshrine {
            chests[6].extend(&sets[9][0..4]);
            chests[7].extend(&sets[9][4..8]);
        }

        // DLC
        if self.unlocks.glacier {
            chests[3].extend(&sets[10][0..4]);
            chests[4].extend(&sets[10][4..8]);
        }
        if self.unlocks.memory {
            chests[3].extend(&sets[11][0..4]);
            chests[5].extend(&sets[11][4..8]);
        }
        if self.unlocks.cultist {
            chests[3].extend(&sets[12][0..4]);
            chests[6].extend(&sets[12][4..8]);
        }
        if self.unlocks.painters {
            chests[3].extend(&sets[13][0..4]);
            chests[7].extend(&sets[13][4..8]);
        }
        if self.unlocks.daynight {
            for i in 0..4 {
                if self.math_coinflip() {
                    chests[5].push(sets[14][i * 2])
                } else {
                    chests[4].push(sets[14][i * 2 + 1])
                }
            }
        }
        if self.unlocks.sharpedge {
            chests[4].extend(&sets[15][0..4]);
            chests[6].extend(&sets[15][4..8]);
        }
        if self.unlocks.oceans {
            chests[4].extend(&sets[16][0..4]);
            chests[7].extend(&sets[16][4..8]);
        }
        if self.unlocks.performers {
            chests[5].extend(&sets[17][0..4]);
            chests[6].extend(&sets[17][4..8]);
        }
        if self.unlocks.miners {
            chests[5].extend(&sets[18][0..4]);
            chests[7].extend(&sets[18][4..8]);
        }
        if self.unlocks.teaparty {
            chests[6].extend(&sets[19][0..4]);
            chests[7].extend(&sets[19][4..8]);
        }
    }

    fn generate_shop(
        self: &mut Self,
        area_index: usize,
        shop_seed: Real,
        all_list: &mut [Vec<usize>; 34],
    ) {
        self.rand.set_seed(shop_seed);

        // Potions
        let mut potion_list = all_list[29].clone();
        self.rand.ds_list_shuffle(&mut potion_list);
        if area_index == 0 && self.high_difficulty {
            potion_list[0] = 681;
        }
        let mut potion_iter = potion_list
            .into_iter()
            .filter(|&potion_id| !(area_index >= 2 && potion_id == 690))
            .enumerate()
            .map(|(i, potion_id)| {
                let price = if i == 0 && area_index == 0 && self.high_difficulty {
                    8 // Regen pot always costs 8
                } else {
                    self.rand.irandom_range(7.into(), 10.into()).into()
                };
                (potion_id, price)
            })
            .map(|(potion_id, price)| Potion::from_id_price(potion_id, price).unwrap())
            .take(3);
        let potions = array::from_fn(|_| potion_iter.next().unwrap());

        // Gems
        let mut gem_lists = [
            all_list[30].clone(),
            all_list[31].clone(),
            all_list[32].clone(),
            all_list[33].clone(),
        ];
        let gems = array::from_fn(|i| {
            self.rand.ds_list_shuffle(&mut gem_lists[i]);
            let gem_id = gem_lists[i][0];
            let price = self.rand.irandom_range(23.into(), 27.into()).into();
            Gem::from_id_price(gem_id, price).unwrap()
        });

        self.shops[area_index] = Some(Shop::new(gems, potions));
    }

    // From RnS source
    fn math_random_switch<T: Copy>(self: &mut Self, list: &[T]) -> Option<T> {
        if list.len() == 0 {
            return None;
        }
        let rand_index: usize = self.rand.irandom((list.len() - 1).into()).into();
        if rand_index >= list.len() {
            return None;
        }
        Some(list[rand_index])
    }

    fn math_coinflip(self: &mut Self) -> bool {
        let rand = Into::<f64>::into(self.rand.random(100.0.into()));

        // In RnS, which comparison is chosen depends on the arg to math_coinflip
        // I don't understand how the arg matters and I find no cases of args other than undef
        if true { rand < 50.0 } else { rand >= 50.0 }
    }

    fn math_random_sign(self: &mut Self) -> Real {
        if Into::<f64>::into(self.rand.random(100.0.into())) >= 50.0 {
            1.into()
        } else {
            (-1).into()
        }
    }

    pub fn predict_seed(self: &mut Self) {
        let mut all_list = Run::get_all_items_list();

        // See `scr_hallwayprogress_shuffle_encounters`
        let mut encounter_lists =
            self.get_shuffled_encounter_list(self.starting_area == StartingArea::ChaoticRandom);
        self.fill_area_list();

        // Items, see `scr_hallwayprogress_shuffle_items`
        self.rand.set_seed((self.map_seed + 5).into());

        self.fill_color_chests(&mut all_list);
        // `scr_hallwayprogress_add_alltreasure` happens between here?

        for list in all_list.iter_mut() {
            self.rand.ds_list_shuffle(list);
        }

        // all_list up to and incl white chest, and the rest
        let (white_chest, other_chests) = all_list.split_at_mut(3);
        for i in 0..5 {
            white_chest[2].extend(other_chests[i].drain(0..8));
        }

        for list in all_list.iter_mut() {
            self.rand.ds_list_shuffle(list);
        }

        let mut chest_types = [2, 2, 2, 3, 4, 5, 6, 7];
        // Yes, we do shuffle twice, not a typo
        self.rand.ds_list_shuffle(&mut chest_types);
        self.rand.ds_list_shuffle(&mut chest_types);

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
                while chest_index >= 4 && item_id == 469
                    || chest_index >= 5 && (item_id == 499 || item_id == 492 || item_id == 495)
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

        // Intro area
        self.rand.set_seed(self.hallseeds[0].into());
        self.hallwaygen_intro(&mut encounter_lists);

        // Areas
        for area_index in 0..3 {
            self.rand.set_seed(self.hallseeds[area_index + 1].into());
            let shop_seed = self.hallwaygen_mid(area_index, &mut encounter_lists);
            self.generate_shop(area_index, shop_seed, &mut all_list);
        }

        // End area
        self.rand.set_seed(self.hallseeds[4].into());
        let shop_seed = self.hallwaygen_end(&mut encounter_lists);
        self.generate_shop(3, shop_seed, &mut all_list);
    }

    pub fn get_csv_line(self: &Self) -> String {
        let mut out = String::new();

        let area_string = self
            .area_list
            .iter()
            .map(|&area| area.to_int().to_string())
            .collect::<Vec<_>>()
            .join(",");
        // TODO: Maybe add, idk it takes so much space
        // let enc_string = self
        //     .encounters
        //     .iter()
        //     .map(|&encounter_index| names::get_area_name(area_index).unwrap())
        //     .collect::<Vec<_>>()
        //     .join(",");
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
        // let outskirt_string = self
        //     .outskirts
        //     .iter()
        //     .map(|&(fight_index, pattern_index)| {
        //         // Write outskirt id where the ten's digit is the fight (1-5) and one's digit is the pattern (1-2)
        //         ((fight_index + 1) * 10 + pattern_index + 1).to_string()
        //     })
        //     .take(3) // Only output 3 fights
        //     .collect::<Vec<_>>()
        //     .join(",");
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
        // out.push_str(&outskirt_string);
        // out.push(',');
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
            .map(|&area| area.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let enc_string = self
            .encounters
            .iter()
            .map(|&encounter| encounter.to_string())
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
        writeln!(f, "Starting area: {:?}", self.starting_area)?;
        writeln!(f, "Areas: [{}]", area_string)?;
        writeln!(f, "Encounters: [{}]", enc_string)?;

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

// Get enough state to tell completely whether two seeds will be equal
// I.e. calculate the first run of the TLCG for seed and seed+5
pub const fn get_short_state(seed: u32, starting_area: StartingArea) -> (u32, u32) {
    let add = match starting_area {
        StartingArea::RandomKingdom => 1,
        StartingArea::RandomExtra => 2,
        StartingArea::TrueRandom => 3,
        StartingArea::ChaoticRandom => 4,
        _ => 0,
    };
    (
        (seed
            .wrapping_add(add)
            .wrapping_mul(0x343fd)
            .wrapping_add(0x269ec3))
            >> 16,
        (seed
            .wrapping_add(5)
            .wrapping_mul(0x343fd)
            .wrapping_add(0x269ec3))
            >> 16,
    )
}
