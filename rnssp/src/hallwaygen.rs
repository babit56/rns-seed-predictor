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
    pub fn hallwaygen_intro(self: &mut Self) {
        let encs = match (self.area_list[0], self.starting_area) {
            (_, ChaoticRandom) => self.hallwaygen_intro_chaos(),
            (Area::Outskirts, _) => self.hallwaygen_outskirts(),
            (Area::Geode, _) => self.hallwaygen_geode(),
            (x, _) => panic!("Intro area should not be: {}", x),
        };
        self.encounters[0..3].copy_from_slice(&encs);
    }

    pub fn hallwaygen_mid(self: &mut Self, area_index: usize) -> Real {
        let (encs, shop_seed) = match (self.area_list[1 + area_index], self.starting_area) {
            (_, ChaoticRandom) => self.hallwaygen_mid_chaos(),
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

    pub fn hallwaygen_end(self: &mut Self) -> Real {
        let (encs, shop_seed) = match (self.area_list[4], self.starting_area) {
            (_, ChaoticRandom) => self.hallwaygen_end_chaos(),
            (Area::Keep, _) => self.hallwaygen_keep(),
            (Area::Darkhall, _) => self.hallwaygen_darkhall(),
            (x, _) => panic!("End area should not be {}", x),
        };
        self.encounters[15..18].copy_from_slice(&encs);
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
        todo!()
    }

    fn hallwaygen_intro_chaos(self: &mut Self) -> [Encounter; 3] {
        todo!()
    }

    fn hallwaygen_mid_chaos(self: &mut Self) -> ([Encounter; 4], Real) {
        todo!()
    }

    fn hallwaygen_end_chaos(self: &mut Self) -> ([Encounter; 4], Real) {
        todo!()
    }
}
