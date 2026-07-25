use gamemaker_rand::{GMRand, Real};

use crate::{
    Run,
    types::{
        Area,
        Encounter::{self, *},
        StartingArea::*,
    },
};

impl Run {
    pub fn hallwaygen_intro(self: &mut Self, encounter_lists: &mut [Vec<Encounter>; 9]) {
        let encs = match (self.area_list[0], self.starting_area) {
            (_, ChaoticRandom) => self.hallwaygen_intro_chaos(encounter_lists),
            (Area::Outskirts, _) => self.hallwaygen_outskirts(),
            (Area::Geode, _) => self.hallwaygen_geode(),
            (x, _) => panic!("Intro area should not be: {}", x),
        };
        self.encounters[0..3].copy_from_slice(&encs);
    }

    pub fn hallwaygen_mid(
        self: &mut Self,
        area_index: usize,
        encounter_lists: &mut [Vec<Encounter>; 9],
    ) -> Real {
        let (encs, shop_seed) = match (self.area_list[1 + area_index], self.starting_area) {
            (_, ChaoticRandom) => self.hallwaygen_mid_chaos(encounter_lists),
            (Area::Nest, _) => self.hallwaygen_nest(),
            (Area::Arsenal, _) => self.hallwaygen_arsenal(),
            (Area::Lighthouse, _) => self.hallwaygen_lighthouse(),
            (Area::Streets, _) => self.hallwaygen_streets(),
            (Area::Lakeside, _) => self.hallwaygen_lakeside(),
            (Area::Sanct, _) => self.hallwaygen_sanct(),
            (Area::Depths, _) => self.hallwaygen_depths(),
            (Area::Aurum, _) => self.hallwaygen_aurum(),
            (x, _) => panic!("Area should not be {}", x),
        };
        let index = 3 + area_index * 4;
        self.encounters[index..(index + 4)].copy_from_slice(&encs);
        return shop_seed;
    }

    pub fn hallwaygen_end(self: &mut Self, encounter_lists: &mut [Vec<Encounter>; 9]) -> Real {
        let (encs, shop_seed) = match (self.area_list[4], self.starting_area) {
            (x, ChaoticRandom) => self.hallwaygen_end_chaos(encounter_lists, x == Area::Keep),
            (Area::Keep, _) => self.hallwaygen_keep(),
            (Area::Darkhall, _) => self.hallwaygen_darkhall(),
            (x, _) => panic!("End area should not be {}", x),
        };
        self.encounters[15..19].copy_from_slice(&encs);
        return shop_seed;
    }

    fn hallwaygen_outskirts(self: &mut Self) -> [Encounter; 3] {
        if !self.high_difficulty {
            return self.hallwaygen_outskirts_normal();
        }
        let mut outskirts = [
            self.math_random_switch(&[BirdSophomore1, BirdSophomore2])
                .unwrap(),
            self.math_random_switch(&[FrogTinkerer1, FrogTinkerer2])
                .unwrap(),
            self.math_random_switch(&[DragonGranite1, DragonGranite2])
                .unwrap(),
            self.math_random_switch(&[WolfBlackear1, WolfBlackear2])
                .unwrap(),
            self.math_random_switch(&[MouseCadet1, MouseCadet2])
                .unwrap(),
        ];
        self.rand.ds_list_shuffle(&mut outskirts);
        return [outskirts[0], outskirts[1], outskirts[2]];
    }

    fn hallwaygen_outskirts_normal(self: &mut Self) -> [Encounter; 3] {
        let mut encs1 = [
            self.math_random_switch(&[BirdSophomore1, BirdSophomore2])
                .unwrap(),
            self.math_random_switch(&[FrogTinkerer1, FrogTinkerer2])
                .unwrap(),
        ];
        let mut encs2 = [
            self.math_random_switch(&[DragonGranite1, DragonGranite2])
                .unwrap(),
            self.math_random_switch(&[WolfBlackear1, WolfBlackear2])
                .unwrap(),
            self.math_random_switch(&[MouseCadet1, MouseCadet2])
                .unwrap(),
        ];
        self.rand.ds_list_shuffle(&mut encs1);
        self.rand.ds_list_shuffle(&mut encs2);
        return [encs1[0], encs2[0], encs2[1]];
    }

    fn hallwaygen_geode(self: &mut Self) -> [Encounter; 3] {
        let mut geode = [
            self.math_random_switch(&[GeodeMoth0, GeodeMoth1, GeodeMoth2])
                .unwrap(),
            self.math_random_switch(&[GeodeButterfly0, GeodeButterfly1, GeodeButterfly2])
                .unwrap(),
            self.math_random_switch(&[GeodeFirefly0, GeodeFirefly1, GeodeFirefly2])
                .unwrap(),
        ];
        self.rand.ds_list_shuffle(&mut geode);

        return geode;
    }

    fn hallwaygen_nest(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        // IDK what this does
        self.math_random_switch(&[8.0, 16.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[BirdStudent0, BirdStudent1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[BirdWhispering0, BirdWhispering1])
            .unwrap();
        // Actual code
        // self.rand.random_range(0.0.into(), 2147483647.0.into());
        // encs[2] = self.math_random_switch(&[BirdArchon0]).unwrap();
        // self.rand.random_range(0.0.into(), 2147483647.0.into());
        // self.rand.random_range(0.0.into(), 2147483647.0.into());
        // encs[3] = self.math_random_switch(&[BirdValedictorian0]).unwrap();
        // self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[2] = BirdArchon0;
        encs[3] = BirdValedictorian0;

        (encs, shop_seed)
    }

    fn hallwaygen_arsenal(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[WolfGreyeye0, WolfGreyeye1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[WolfBluepaw0, WolfBluepaw1])
            .unwrap();
        encs[2] = WolfSnowfur0;
        encs[3] = WolfSteeltooth0;

        (encs, shop_seed)
    }

    fn hallwaygen_lighthouse(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[DragonGold0, DragonGold1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[DragonEmerald0, DragonEmerald1])
            .unwrap();
        encs[2] = DragonRuby0;
        encs[3] = DragonMythril0;

        (encs, shop_seed)
    }

    fn hallwaygen_streets(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[MouseArcher0, MouseArcher1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[MouseOakspear0, MouseOakspear1])
            .unwrap();
        encs[2] = MouseRosemage0;
        encs[3] = MousePaladin0;

        (encs, shop_seed)
    }

    fn hallwaygen_lakeside(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[FrogSeamstress0, FrogSeamstress1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[FrogSongstress0, FrogSongstress1])
            .unwrap();
        encs[2] = FrogPainter0;
        encs[3] = FrogIdol0;

        (encs, shop_seed)
    }

    fn hallwaygen_sanct(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        self.math_random_switch(&[16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[SanctCapricorn0, SanctCapricorn1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self.math_random_switch(&[SanctSaph0, SanctSaph1]).unwrap();
        encs[2] = SanctFlower0;
        encs[3] = SanctOwl0;

        (encs, shop_seed)
    }

    fn hallwaygen_depths(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        self.math_random_switch(&[16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[DepthsBasilisk0, DepthsBasilisk1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[DepthsBeast0, DepthsBeast1])
            .unwrap();
        encs[2] = DepthsAngel0;
        encs[3] = DepthsHound0;

        (encs, shop_seed)
    }

    fn hallwaygen_aurum(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        self.math_random_switch(&[16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        encs[0] = self
            .math_random_switch(&[AurumWhitecat0, AurumWhitecat1])
            .unwrap();
        self.rand.random_range(0.0.into(), 2147483647.0.into());
        encs[1] = self
            .math_random_switch(&[AurumBeast0, AurumBeast1])
            .unwrap();
        encs[2] = AurumGhost0;
        encs[3] = AurumBlackcat0;

        (encs, shop_seed)
    }

    fn hallwaygen_keep(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut keep = [
            QueensStaff0,
            QueensKnife0,
            QueensAxe0,
            QueensSpear0,
            QueensHarp0,
        ];
        self.rand.ds_list_shuffle(&mut keep);
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        let mut encs = [TrainingNothing; 4];
        encs[0..3].copy_from_slice(&keep[0..3]);
        encs[3] = RabbitQueen0;
        (encs, shop_seed)
    }

    fn hallwaygen_darkhall(self: &mut Self) -> ([Encounter; 4], Real) {
        let mut darkhall = [
            DarkhallSpelllock0,
            DarkhallSpelllock1,
            DarkhallSpelllock2,
            DarkhallSpelllock3,
            DarkhallSpelllock4,
        ];
        self.rand.ds_list_shuffle(&mut darkhall);
        self.math_random_switch(&[8.0, 16.0, 32.0]).unwrap();
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());

        let mut encs = [TrainingNothing; 4];
        encs[0..3].copy_from_slice(&darkhall[0..3]);
        encs[3] = HeartWitch0;
        (encs, shop_seed)
    }

    fn hallwaygen_intro_chaos(
        self: &mut Self,
        encounter_lists: &mut [Vec<Encounter>; 9],
    ) -> [Encounter; 3] {
        let possible_fights = &mut encounter_lists[2];
        let mut encs = [TrainingNothing; 3];
        for i in 0..3 {
            encs[i] = possible_fights.pop().unwrap();
            let to_remove = match encs[i] {
                x if [BirdSophomore1, BirdSophomore2].contains(&x) => {
                    vec![BirdSophomore1, BirdSophomore2]
                }
                x if [WolfBlackear1, WolfBlackear2].contains(&x) => {
                    vec![WolfBlackear1, WolfBlackear2]
                }
                x if [DragonGranite1, DragonGranite2].contains(&x) => {
                    vec![DragonGranite1, DragonGranite2]
                }
                x if [MouseCadet1, MouseCadet2].contains(&x) => vec![MouseCadet1, MouseCadet2],
                x if [FrogTinkerer1, FrogTinkerer2].contains(&x) => {
                    vec![FrogTinkerer1, FrogTinkerer2]
                }
                x if [GeodeMoth0, GeodeMoth1, GeodeMoth2].contains(&x) => {
                    vec![GeodeMoth0, GeodeMoth1, GeodeMoth2]
                }
                x if [GeodeFirefly0, GeodeFirefly1, GeodeFirefly2].contains(&x) => {
                    vec![GeodeFirefly0, GeodeFirefly1, GeodeFirefly2]
                }
                x if [GeodeButterfly0, GeodeButterfly1, GeodeButterfly2].contains(&x) => {
                    vec![GeodeButterfly0, GeodeButterfly1, GeodeButterfly2]
                }
                x => panic!("Intro fight should not be: {}", x),
            };
            for enc in to_remove {
                if let Some(ind) = possible_fights.iter().position(|&x| x == enc) {
                    possible_fights.remove(ind);
                }
            }
        }

        encs
    }

    fn hallwaygen_mid_chaos(
        self: &mut Self,
        encounter_lists: &mut [Vec<Encounter>; 9],
    ) -> ([Encounter; 4], Real) {
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8, 16, 32]);
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());
        for i in 0..2 {
            let possible_fights = if self.math_coinflip() {
                &mut encounter_lists[3]
            } else {
                &mut encounter_lists[4]
            };
            encs[i] = possible_fights.pop().unwrap();
            self.rand.random_range(0.0.into(), 2147483647.0.into());
        }
        encs[2] = encounter_lists[5].pop().unwrap();
        encs[3] = encounter_lists[6].pop().unwrap();

        (encs, shop_seed)
    }

    fn hallwaygen_end_chaos(
        self: &mut Self,
        encounter_lists: &mut [Vec<Encounter>; 9],
        is_keep: bool,
    ) -> ([Encounter; 4], Real) {
        let possible_fights = &mut encounter_lists[7];
        let mut encs = [TrainingNothing; 4];
        self.math_random_switch(&[8, 16, 32]);
        let shop_seed = self.rand.random_range(0.0.into(), 2147483647.0.into());
        for i in 0..3 {
            encs[i] = possible_fights.pop().unwrap();
        }
        if is_keep {
            encs[3] = RabbitQueen0;
        } else {
            encs[3] = HeartWitch0;
        }
        (encs, shop_seed)
    }

    pub fn get_shuffled_encounter_list(self: &mut Self, is_chaos: bool) -> [Vec<Encounter>; 9] {
        // See NOTES.md for information the contents of each list
        let mut lists = [
            vec![
                Encounter::TrainingNothing,
                Encounter::TrainingCircles,
                Encounter::TrainingCircleSpreads,
                Encounter::TrainingCones,
                Encounter::TrainingConeSpreads,
                Encounter::TrainingLine,
                Encounter::TrainingLineSpreads,
                Encounter::TrainingPrscircle,
                Encounter::TrainingLinestack,
                Encounter::TrainingLinestackFollow,
                Encounter::TrainingSpinray,
                Encounter::TrainingSpinfast,
                Encounter::TrainingEnlargeBullet,
                Encounter::TrainingEnlargeRay,
                Encounter::TrainingColormatch,
                Encounter::TrainingClockspot,
                Encounter::TrainingCardinal,
                Encounter::TrainingCleave,
                Encounter::TrainingTether,
                Encounter::TrainingTetherFixed,
                Encounter::TrainingMovecheck,
                Encounter::TrainingThorns,
                Encounter::TrainingThornsFixed,
                Encounter::TrainingMarching,
                Encounter::TrainingBind,
                Encounter::TrainingGravity,
                Encounter::TrainingHeavy,
                Encounter::TrainingTailwind,
                Encounter::TrainingTeleport,
                Encounter::TrainingTimestop,
                Encounter::TrainingFieldlimit,
                Encounter::TrainingFieldlimit2,
                Encounter::TrainingDisplayorder,
                Encounter::TrainingEnrage,
                Encounter::TrainingTransform,
                Encounter::TrainingTransformMinor,
                Encounter::TrainingDark0,
                Encounter::TrainingDark1,
                Encounter::TrainingDark2,
                Encounter::TrainingSteel0,
                Encounter::TrainingSteel1,
                Encounter::TrainingFire0,
                Encounter::TrainingFire1,
                Encounter::TrainingLight0,
                Encounter::TrainingLight1,
                Encounter::TrainingWater0,
                Encounter::TrainingWater1,
                Encounter::TrainingSpinningCardinal,
                Encounter::TrainingAngelCircle,
                Encounter::TrainingDeathwall,
                Encounter::TrainingTargetTether,
                Encounter::TrainingSummonButterfly,
                Encounter::TrainingSummonFishpool,
                Encounter::TrainingSummonJellycat,
                Encounter::TrainingBloomRing,
                Encounter::TrainingBlackwhiteSpawn,
                Encounter::TrainingPlayermirror,
                Encounter::TrainingSummonAurumsword,
                Encounter::TrainingGemPick,
                Encounter::TrainingAngelAppear,
                Encounter::TrainingPatternErase,
                Encounter::BirdValedictorian1,
                Encounter::WolfSteeltooth1,
                Encounter::DragonMythril1,
                Encounter::MousePaladin1,
                Encounter::FrogIdol1,
                Encounter::RabbitQueen1,
                Encounter::DepthsHound1,
                Encounter::SanctOwl1,
                Encounter::AurumBlackcat1,
                Encounter::HeartWitch1,
                Encounter::ToyboxSphere,
                Encounter::ToyboxSphereAttack,
                Encounter::ToyboxSphereMove,
                Encounter::ToyboxSphereTwo,
                Encounter::ToyboxSphereFour,
                Encounter::ToyboxSphereSummon,
            ],
            vec![
                Encounter::BirdSophomore0,
                Encounter::WolfBlackear0,
                Encounter::DragonGranite0,
                Encounter::MouseCadet0,
                Encounter::FrogTinkerer0,
            ],
            vec![
                Encounter::BirdSophomore1,
                Encounter::BirdSophomore2,
                Encounter::WolfBlackear1,
                Encounter::WolfBlackear2,
                Encounter::DragonGranite1,
                Encounter::DragonGranite2,
                Encounter::MouseCadet1,
                Encounter::MouseCadet2,
                Encounter::FrogTinkerer1,
                Encounter::FrogTinkerer2,
                Encounter::GeodeMoth0,
                Encounter::GeodeMoth1,
                Encounter::GeodeMoth2,
                Encounter::GeodeFirefly0,
                Encounter::GeodeFirefly1,
                Encounter::GeodeFirefly2,
                Encounter::GeodeButterfly0,
                Encounter::GeodeButterfly1,
                Encounter::GeodeButterfly2,
            ],
            vec![
                Encounter::BirdStudent0,
                Encounter::BirdStudent1,
                Encounter::WolfGreyeye0,
                Encounter::WolfGreyeye1,
                Encounter::DragonGold0,
                Encounter::DragonGold1,
                Encounter::MouseArcher0,
                Encounter::MouseArcher1,
                Encounter::FrogSeamstress0,
                Encounter::FrogSeamstress1,
                Encounter::DepthsBasilisk0,
                Encounter::DepthsBasilisk1,
                Encounter::SanctSaph0,
                Encounter::SanctSaph1,
                Encounter::AurumWhitecat0,
                Encounter::AurumWhitecat1,
            ],
            vec![
                Encounter::BirdWhispering0,
                Encounter::BirdWhispering1,
                Encounter::WolfBluepaw0,
                Encounter::WolfBluepaw1,
                Encounter::DragonEmerald0,
                Encounter::DragonEmerald1,
                Encounter::MouseOakspear0,
                Encounter::MouseOakspear1,
                Encounter::FrogSongstress0,
                Encounter::FrogSongstress1,
                Encounter::DepthsBeast0,
                Encounter::DepthsBeast1,
                Encounter::SanctCapricorn0,
                Encounter::SanctCapricorn1,
                Encounter::AurumBeast0,
                Encounter::AurumBeast1,
            ],
            vec![
                Encounter::BirdArchon0,
                Encounter::WolfSnowfur0,
                Encounter::DragonRuby0,
                Encounter::MouseRosemage0,
                Encounter::FrogPainter0,
                Encounter::DepthsAngel0,
                Encounter::SanctFlower0,
                Encounter::AurumGhost0,
            ],
            vec![
                Encounter::BirdValedictorian0,
                Encounter::WolfSteeltooth0,
                Encounter::DragonMythril0,
                Encounter::MousePaladin0,
                Encounter::FrogIdol0,
                Encounter::DepthsHound0,
                Encounter::SanctOwl0,
                Encounter::AurumBlackcat0,
            ],
            vec![
                Encounter::QueensStaff0,
                Encounter::QueensKnife0,
                Encounter::QueensAxe0,
                Encounter::QueensSpear0,
                Encounter::QueensHarp0,
                Encounter::DarkhallSpelllock0,
                Encounter::DarkhallSpelllock1,
                Encounter::DarkhallSpelllock2,
                Encounter::DarkhallSpelllock3,
                Encounter::DarkhallSpelllock4,
            ],
            vec![Encounter::RabbitQueen0, Encounter::HeartWitch0],
        ];
        if is_chaos {
            self.rand.set_seed((self.map_seed + 3).into());
            for list in lists.iter_mut() {
                self.rand.ds_list_shuffle(list);
                list.reverse(); // Reverse so we can .pop() out elements
            }
        }
        lists
    }
}
