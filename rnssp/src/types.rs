#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChestColor {
    WHITE = 2,
    OPAL = 3,
    SAPPHIRE = 4,
    RUBY = 5,
    GARNET = 6,
    EMERALD = 7,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chest {
    pub color: ChestColor,
    pub items: Vec<usize>,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GemType {
    OPAL,
    SAPPHIRE,
    RUBY,
    GARNET,
    EMERALD,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gem {
    pub gem_id: usize,
    pub gem_type: GemType,
    pub price: usize,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Potion {
    pub potion_id: usize,
    pub price: usize,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shop {
    pub gems: [Gem; 4],
    pub potions: [Potion; 3],
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unlocks {
    pub darkbite: bool,
    pub timegem: bool,
    pub youkai: bool,
    pub haunted: bool,
    pub gladiator: bool,
    pub sparkblade: bool,
    pub swiftflight: bool,
    pub sacredflame: bool,
    pub ruins: bool,
    pub lakeshrine: bool,
    pub glacier: bool,
    pub memory: bool,
    pub cultist: bool,
    pub painters: bool,
    pub daynight: bool,
    pub sharpedge: bool,
    pub oceans: bool,
    pub performers: bool,
    pub miners: bool,
    pub teaparty: bool,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Area {
//     pub encounters: [usize; 4],
//     pub shop: Shop,
//     pub chest: Chest,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartingArea {
    RandomKingdom,
    RandomExtra,
    TrueRandom,
    ChaoticRandom,
    Nest,
    Arsenal,
    Lighthouse,
    Streets,
    Lakeside,
    Sanct,
    Depths,
    Aurum,
}

impl TryFrom<usize> for ChestColor {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::WHITE),
            3 => Ok(Self::OPAL),
            4 => Ok(Self::SAPPHIRE),
            5 => Ok(Self::RUBY),
            6 => Ok(Self::GARNET),
            7 => Ok(Self::EMERALD),
            _ => Err("Got invalid chest color index"),
        }
    }
}

impl fmt::Display for ChestColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_str = match *self {
            ChestColor::WHITE => "White",
            ChestColor::OPAL => "Opal",
            ChestColor::SAPPHIRE => "Sapphire",
            ChestColor::RUBY => "Ruby",
            ChestColor::GARNET => "Garnet",
            ChestColor::EMERALD => "Emerald",
        };
        write!(f, "{}", color_str)
    }
}

impl Chest {
    pub fn from_id(color_id: usize, items: Vec<usize>) -> Result<Self, &'static str> {
        let color = ChestColor::try_from(color_id)?;
        Ok(Self { color, items })
    }
}

impl TryFrom<usize> for GemType {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OPAL),
            1 => Ok(Self::SAPPHIRE),
            2 => Ok(Self::RUBY),
            3 => Ok(Self::GARNET),
            4 => Ok(Self::EMERALD),
            _ => Err("Got invalid chest color index"),
        }
    }
}

impl fmt::Display for GemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_str = match *self {
            GemType::OPAL => "Opal",
            GemType::SAPPHIRE => "Sapphire",
            GemType::RUBY => "Ruby",
            GemType::GARNET => "Garnet",
            GemType::EMERALD => "Emerald",
        };
        write!(f, "{}", color_str)
    }
}

impl Gem {
    pub fn from_id_price(gem_id: usize, price: usize) -> Result<Self, &'static str> {
        if !(696 <= gem_id && gem_id <= 715) {
            return Err("Gem should have ID between 696 and 715");
        }
        if !(23 <= price && price <= 27) {
            return Err("Gem price should be between 23 and 27");
        }
        // Assumes gem upgrade ID's are ordered as Opal (primary) Opal (secondary) Opal Opal, Sapphire ...
        let gem_type = GemType::try_from((gem_id - 696) / 4)?;
        Ok(Self {
            gem_id,
            gem_type,
            price,
        })
    }
}

impl Potion {
    pub fn from_id_price(potion_id: usize, price: usize) -> Result<Self, &'static str> {
        if !(681 <= potion_id && potion_id <= 695) {
            return Err("Potion should have ID between 681 and 695");
        }
        if !(7 <= price && price <= 10) {
            return Err("Potion price should be between 7 and 10");
        }
        Ok(Self { potion_id, price })
    }
}

impl Shop {
    pub fn new(gems: [Gem; 4], potions: [Potion; 3]) -> Self {
        Self { gems, potions }
    }
}

impl Unlocks {
    pub fn with_none() -> Self {
        Self {
            darkbite: false,
            timegem: false,
            youkai: false,
            haunted: false,
            gladiator: false,
            sparkblade: false,
            swiftflight: false,
            sacredflame: false,
            ruins: false,
            lakeshrine: false,
            glacier: false,
            memory: false,
            cultist: false,
            painters: false,
            daynight: false,
            sharpedge: false,
            oceans: false,
            performers: false,
            miners: false,
            teaparty: false,
        }
    }

    pub fn with_all() -> Self {
        Self {
            darkbite: true,
            timegem: true,
            youkai: true,
            haunted: true,
            gladiator: true,
            sparkblade: true,
            swiftflight: true,
            sacredflame: true,
            ruins: true,
            lakeshrine: true,
            glacier: true,
            memory: true,
            cultist: true,
            painters: true,
            daynight: true,
            sharpedge: true,
            oceans: true,
            performers: true,
            miners: true,
            teaparty: true,
        }
    }

    /// Interprets usize as a bitstring with each bit corresponding to a field
    #[rustfmt::skip]
    pub fn from_bitstring(bitstring: usize) -> Self {
        Self {
            darkbite:    bitstring & (1 << 0) != 0,
            timegem:     bitstring & (1 << 1) != 0,
            youkai:      bitstring & (1 << 2) != 0,
            haunted:     bitstring & (1 << 3) != 0,
            gladiator:   bitstring & (1 << 4) != 0,
            sparkblade:  bitstring & (1 << 5) != 0,
            swiftflight: bitstring & (1 << 6) != 0,
            sacredflame: bitstring & (1 << 7) != 0,
            ruins:       bitstring & (1 << 8) != 0,
            lakeshrine:  bitstring & (1 << 9) != 0,
            glacier:     bitstring & (1 << 10) != 0,
            memory:      bitstring & (1 << 11) != 0,
            cultist:     bitstring & (1 << 12) != 0,
            painters:    bitstring & (1 << 13) != 0,
            daynight:    bitstring & (1 << 14) != 0,
            sharpedge:   bitstring & (1 << 15) != 0,
            oceans:      bitstring & (1 << 16) != 0,
            performers:  bitstring & (1 << 17) != 0,
            miners:      bitstring & (1 << 18) != 0,
            teaparty:    bitstring & (1 << 19) != 0,
        }
    }

    #[rustfmt::skip]
    pub fn get_bitstring(self: &Self) -> usize {
        let mut bitmask = 0;

        if self.darkbite    { bitmask |= 1 << 0 }
        if self.timegem     { bitmask |= 1 << 1 }
        if self.youkai      { bitmask |= 1 << 2 }
        if self.haunted     { bitmask |= 1 << 3 }
        if self.gladiator   { bitmask |= 1 << 4 }
        if self.sparkblade  { bitmask |= 1 << 5 }
        if self.swiftflight { bitmask |= 1 << 6 }
        if self.sacredflame { bitmask |= 1 << 7 }
        if self.ruins       { bitmask |= 1 << 8 }
        if self.lakeshrine  { bitmask |= 1 << 9 }
        if self.glacier     { bitmask |= 1 << 10 }
        if self.memory      { bitmask |= 1 << 11 }
        if self.cultist     { bitmask |= 1 << 12 }
        if self.painters    { bitmask |= 1 << 13 }
        if self.daynight    { bitmask |= 1 << 14 }
        if self.sharpedge   { bitmask |= 1 << 15 }
        if self.oceans      { bitmask |= 1 << 16 }
        if self.performers  { bitmask |= 1 << 17 }
        if self.miners      { bitmask |= 1 << 18 }
        if self.teaparty    { bitmask |= 1 << 19 }

        bitmask
    }
}

impl StartingArea {
    /// Is Kingdom Random or one of the kingdom areas
    pub fn is_kingdom(self: &Self) -> bool {
        match self {
            StartingArea::RandomKingdom => true,
            StartingArea::Nest => true,
            StartingArea::Arsenal => true,
            StartingArea::Lighthouse => true,
            StartingArea::Streets => true,
            StartingArea::Lakeside => true,
            _ => false,
        }
    }

    /// Is Extra Random or one of the extra areas
    pub fn is_extra(self: &Self) -> bool {
        match self {
            StartingArea::RandomExtra => true,
            StartingArea::Sanct => true,
            StartingArea::Depths => true,
            StartingArea::Aurum => true,
            _ => false,
        }
    }

    pub fn is_random(self: &Self) -> bool {
        match self {
            StartingArea::RandomKingdom => true,
            StartingArea::RandomExtra => true,
            StartingArea::TrueRandom => true,
            StartingArea::ChaoticRandom => true,
            _ => false,
        }
    }

    pub fn is_very_random(self: &Self) -> bool {
        match self {
            StartingArea::TrueRandom => true,
            StartingArea::ChaoticRandom => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    Outskirts,
    Nest,
    Arsenal,
    Lighthouse,
    Streets,
    Lakeside,
    Keep,
    Geode,
    Sanct,
    Depths,
    Aurum,
    Darkhall,
}

// TODO: Use debug impl for these names, display for pretty names
impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Area::Outskirts => "hw_outskirts",
            Area::Nest => "hw_nest",
            Area::Arsenal => "hw_arsenal",
            Area::Lighthouse => "hw_lighthouse",
            Area::Streets => "hw_streets",
            Area::Lakeside => "hw_lakeside",
            Area::Keep => "hw_keep",
            Area::Geode => "hw_geode",
            Area::Sanct => "hw_sanct",
            Area::Depths => "hw_depths",
            Area::Aurum => "hw_aurum",
            Area::Darkhall => "hw_darkhall",
        };
        write!(f, "{}", string)
    }
}

impl Area {
    pub fn to_int(&self) -> usize {
        match self {
            Area::Outskirts => 0,
            Area::Nest => 1,
            Area::Arsenal => 2,
            Area::Lighthouse => 3,
            Area::Streets => 4,
            Area::Lakeside => 5,
            Area::Keep => 6,
            Area::Geode => 7,
            Area::Sanct => 8,
            Area::Depths => 9,
            Area::Aurum => 10,
            Area::Darkhall => 11,
        }
    }
}

impl TryFrom<StartingArea> for Area {
    type Error = &'static str;

    fn try_from(value: StartingArea) -> Result<Self, Self::Error> {
        match value {
            StartingArea::RandomKingdom
            | StartingArea::RandomExtra
            | StartingArea::TrueRandom
            | StartingArea::ChaoticRandom => Err("Could not convert random StartingArea to Area"),
            StartingArea::Nest => Ok(Area::Nest),
            StartingArea::Arsenal => Ok(Area::Arsenal),
            StartingArea::Lighthouse => Ok(Area::Lighthouse),
            StartingArea::Streets => Ok(Area::Streets),
            StartingArea::Lakeside => Ok(Area::Lakeside),
            StartingArea::Sanct => Ok(Area::Sanct),
            StartingArea::Depths => Ok(Area::Depths),
            StartingArea::Aurum => Ok(Area::Aurum),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encounter {
    TrainingNothing,
    TrainingCircles,
    TrainingCircleSpreads,
    TrainingCones,
    TrainingConeSpreads,
    TrainingLine,
    TrainingLineSpreads,
    TrainingPrscircle,
    TrainingLinestack,
    TrainingLinestackFollow,
    TrainingSpinray,
    TrainingSpinfast,
    TrainingEnlargeBullet,
    TrainingEnlargeRay,
    TrainingColormatch,
    TrainingClockspot,
    TrainingCardinal,
    TrainingCleave,
    TrainingTether,
    TrainingTetherFixed,
    TrainingMovecheck,
    TrainingThorns,
    TrainingThornsFixed,
    TrainingMarching,
    TrainingBind,
    TrainingGravity,
    TrainingHeavy,
    TrainingTailwind,
    TrainingTeleport,
    TrainingTimestop,
    TrainingFieldlimit,
    TrainingFieldlimit2,
    TrainingDisplayorder,
    TrainingEnrage,
    TrainingTransform,
    TrainingTransformMinor,
    TrainingDark0,
    TrainingDark1,
    TrainingDark2,
    TrainingSteel0,
    TrainingSteel1,
    TrainingFire0,
    TrainingFire1,
    TrainingLight0,
    TrainingLight1,
    TrainingWater0,
    TrainingWater1,
    TrainingSpinningCardinal,
    TrainingAngelCircle,
    TrainingDeathwall,
    TrainingTargetTether,
    TrainingSummonButterfly,
    TrainingSummonFishpool,
    TrainingSummonJellycat,
    TrainingBloomRing,
    TrainingBlackwhiteSpawn,
    TrainingPlayermirror,
    TrainingSummonAurumsword,
    TrainingGemPick,
    TrainingAngelAppear,
    TrainingPatternErase,
    BirdSophomore0,
    BirdSophomore1,
    BirdSophomore2,
    WolfBlackear0,
    WolfBlackear1,
    WolfBlackear2,
    DragonGranite0,
    DragonGranite1,
    DragonGranite2,
    MouseCadet0,
    MouseCadet1,
    MouseCadet2,
    FrogTinkerer0,
    FrogTinkerer1,
    FrogTinkerer2,
    BirdStudent0,
    BirdStudent1,
    BirdWhispering0,
    BirdWhispering1,
    BirdArchon0,
    BirdValedictorian0,
    BirdValedictorian1,
    WolfGreyeye0,
    WolfGreyeye1,
    WolfBluepaw0,
    WolfBluepaw1,
    WolfSnowfur0,
    WolfSteeltooth0,
    WolfSteeltooth1,
    DragonGold0,
    DragonGold1,
    DragonEmerald0,
    DragonEmerald1,
    DragonRuby0,
    DragonMythril0,
    DragonMythril1,
    MouseArcher0,
    MouseArcher1,
    MouseOakspear0,
    MouseOakspear1,
    MouseRosemage0,
    MousePaladin0,
    MousePaladin1,
    FrogSeamstress0,
    FrogSeamstress1,
    FrogSongstress0,
    FrogSongstress1,
    FrogPainter0,
    FrogIdol0,
    FrogIdol1,
    QueensStaff0,
    QueensKnife0,
    QueensAxe0,
    QueensSpear0,
    QueensHarp0,
    RabbitQueen0,
    RabbitQueen1,
    GeodeMoth0,
    GeodeMoth1,
    GeodeMoth2,
    GeodeFirefly0,
    GeodeFirefly1,
    GeodeFirefly2,
    GeodeButterfly0,
    GeodeButterfly1,
    GeodeButterfly2,
    DepthsBasilisk0,
    DepthsBasilisk1,
    DepthsBeast0,
    DepthsBeast1,
    DepthsAngel0,
    DepthsHound0,
    DepthsHound1,
    SanctSaph0,
    SanctSaph1,
    SanctCapricorn0,
    SanctCapricorn1,
    SanctFlower0,
    SanctOwl0,
    SanctOwl1,
    AurumWhitecat0,
    AurumWhitecat1,
    AurumBeast0,
    AurumBeast1,
    AurumGhost0,
    AurumBlackcat0,
    AurumBlackcat1,
    DarkhallSpelllock0,
    DarkhallSpelllock1,
    DarkhallSpelllock2,
    DarkhallSpelllock3,
    DarkhallSpelllock4,
    HeartWitch0,
    HeartWitch1,
    ToyboxSphere,
    ToyboxSphereAttack,
    ToyboxSphereMove,
}

// TODO: Use debug impl for these names, display for pretty names
impl fmt::Display for Encounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Encounter::TrainingNothing => "enc_training_nothing",
            Encounter::TrainingCircles => "enc_training_circles",
            Encounter::TrainingCircleSpreads => "enc_training_circle_spreads",
            Encounter::TrainingCones => "enc_training_cones",
            Encounter::TrainingConeSpreads => "enc_training_cone_spreads",
            Encounter::TrainingLine => "enc_training_line",
            Encounter::TrainingLineSpreads => "enc_training_line_spreads",
            Encounter::TrainingPrscircle => "enc_training_prscircle",
            Encounter::TrainingLinestack => "enc_training_linestack",
            Encounter::TrainingLinestackFollow => "enc_training_linestack_follow",
            Encounter::TrainingSpinray => "enc_training_spinray",
            Encounter::TrainingSpinfast => "enc_training_spinfast",
            Encounter::TrainingEnlargeBullet => "enc_training_enlarge_bullet",
            Encounter::TrainingEnlargeRay => "enc_training_enlarge_ray",
            Encounter::TrainingColormatch => "enc_training_colormatch",
            Encounter::TrainingClockspot => "enc_training_clockspot",
            Encounter::TrainingCardinal => "enc_training_cardinal",
            Encounter::TrainingCleave => "enc_training_cleave",
            Encounter::TrainingTether => "enc_training_tether",
            Encounter::TrainingTetherFixed => "enc_training_tether_fixed",
            Encounter::TrainingMovecheck => "enc_training_movecheck",
            Encounter::TrainingThorns => "enc_training_thorns",
            Encounter::TrainingThornsFixed => "enc_training_thorns_fixed",
            Encounter::TrainingMarching => "enc_training_marching",
            Encounter::TrainingBind => "enc_training_bind",
            Encounter::TrainingGravity => "enc_training_gravity",
            Encounter::TrainingHeavy => "enc_training_heavy",
            Encounter::TrainingTailwind => "enc_training_tailwind",
            Encounter::TrainingTeleport => "enc_training_teleport",
            Encounter::TrainingTimestop => "enc_training_timestop",
            Encounter::TrainingFieldlimit => "enc_training_fieldlimit",
            Encounter::TrainingFieldlimit2 => "enc_training_fieldlimit2",
            Encounter::TrainingDisplayorder => "enc_training_displayorder",
            Encounter::TrainingEnrage => "enc_training_enrage",
            Encounter::TrainingTransform => "enc_training_transform",
            Encounter::TrainingTransformMinor => "enc_training_transform_minor",
            Encounter::TrainingDark0 => "enc_training_dark0",
            Encounter::TrainingDark1 => "enc_training_dark1",
            Encounter::TrainingDark2 => "enc_training_dark2",
            Encounter::TrainingSteel0 => "enc_training_steel0",
            Encounter::TrainingSteel1 => "enc_training_steel1",
            Encounter::TrainingFire0 => "enc_training_fire0",
            Encounter::TrainingFire1 => "enc_training_fire1",
            Encounter::TrainingLight0 => "enc_training_light0",
            Encounter::TrainingLight1 => "enc_training_light1",
            Encounter::TrainingWater0 => "enc_training_water0",
            Encounter::TrainingWater1 => "enc_training_water1",
            Encounter::TrainingSpinningCardinal => "enc_training_spinning_cardinal",
            Encounter::TrainingAngelCircle => "enc_training_angel_circle",
            Encounter::TrainingDeathwall => "enc_training_deathwall",
            Encounter::TrainingTargetTether => "enc_training_target_tether",
            Encounter::TrainingSummonButterfly => "enc_training_summon_butterfly",
            Encounter::TrainingSummonFishpool => "enc_training_summon_fishpool",
            Encounter::TrainingSummonJellycat => "enc_training_summon_jellycat",
            Encounter::TrainingBloomRing => "enc_training_bloom_ring",
            Encounter::TrainingBlackwhiteSpawn => "enc_training_blackwhite_spawn",
            Encounter::TrainingPlayermirror => "enc_training_playermirror",
            Encounter::TrainingSummonAurumsword => "enc_training_summon_aurumsword",
            Encounter::TrainingGemPick => "enc_training_gem_pick",
            Encounter::TrainingAngelAppear => "enc_training_angel_appear",
            Encounter::TrainingPatternErase => "enc_training_pattern_erase",
            Encounter::BirdSophomore0 => "enc_bird_sophomore0",
            Encounter::BirdSophomore1 => "enc_bird_sophomore1",
            Encounter::BirdSophomore2 => "enc_bird_sophomore2",
            Encounter::WolfBlackear0 => "enc_wolf_blackear0",
            Encounter::WolfBlackear1 => "enc_wolf_blackear1",
            Encounter::WolfBlackear2 => "enc_wolf_blackear2",
            Encounter::DragonGranite0 => "enc_dragon_granite0",
            Encounter::DragonGranite1 => "enc_dragon_granite1",
            Encounter::DragonGranite2 => "enc_dragon_granite2",
            Encounter::MouseCadet0 => "enc_mouse_cadet0",
            Encounter::MouseCadet1 => "enc_mouse_cadet1",
            Encounter::MouseCadet2 => "enc_mouse_cadet2",
            Encounter::FrogTinkerer0 => "enc_frog_tinkerer0",
            Encounter::FrogTinkerer1 => "enc_frog_tinkerer1",
            Encounter::FrogTinkerer2 => "enc_frog_tinkerer2",
            Encounter::BirdStudent0 => "enc_bird_student0",
            Encounter::BirdStudent1 => "enc_bird_student1",
            Encounter::BirdWhispering0 => "enc_bird_whispering0",
            Encounter::BirdWhispering1 => "enc_bird_whispering1",
            Encounter::BirdArchon0 => "enc_bird_archon0",
            Encounter::BirdValedictorian0 => "enc_bird_valedictorian0",
            Encounter::BirdValedictorian1 => "enc_bird_valedictorian1",
            Encounter::WolfGreyeye0 => "enc_wolf_greyeye0",
            Encounter::WolfGreyeye1 => "enc_wolf_greyeye1",
            Encounter::WolfBluepaw0 => "enc_wolf_bluepaw0",
            Encounter::WolfBluepaw1 => "enc_wolf_bluepaw1",
            Encounter::WolfSnowfur0 => "enc_wolf_snowfur0",
            Encounter::WolfSteeltooth0 => "enc_wolf_steeltooth0",
            Encounter::WolfSteeltooth1 => "enc_wolf_steeltooth1",
            Encounter::DragonGold0 => "enc_dragon_gold0",
            Encounter::DragonGold1 => "enc_dragon_gold1",
            Encounter::DragonEmerald0 => "enc_dragon_emerald0",
            Encounter::DragonEmerald1 => "enc_dragon_emerald1",
            Encounter::DragonRuby0 => "enc_dragon_ruby0",
            Encounter::DragonMythril0 => "enc_dragon_mythril0",
            Encounter::DragonMythril1 => "enc_dragon_mythril1",
            Encounter::MouseArcher0 => "enc_mouse_archer0",
            Encounter::MouseArcher1 => "enc_mouse_archer1",
            Encounter::MouseOakspear0 => "enc_mouse_oakspear0",
            Encounter::MouseOakspear1 => "enc_mouse_oakspear1",
            Encounter::MouseRosemage0 => "enc_mouse_rosemage0",
            Encounter::MousePaladin0 => "enc_mouse_paladin0",
            Encounter::MousePaladin1 => "enc_mouse_paladin1",
            Encounter::FrogSeamstress0 => "enc_frog_seamstress0",
            Encounter::FrogSeamstress1 => "enc_frog_seamstress1",
            Encounter::FrogSongstress0 => "enc_frog_songstress0",
            Encounter::FrogSongstress1 => "enc_frog_songstress1",
            Encounter::FrogPainter0 => "enc_frog_painter0",
            Encounter::FrogIdol0 => "enc_frog_idol0",
            Encounter::FrogIdol1 => "enc_frog_idol1",
            Encounter::QueensStaff0 => "enc_queens_staff0",
            Encounter::QueensKnife0 => "enc_queens_knife0",
            Encounter::QueensAxe0 => "enc_queens_axe0",
            Encounter::QueensSpear0 => "enc_queens_spear0",
            Encounter::QueensHarp0 => "enc_queens_harp0",
            Encounter::RabbitQueen0 => "enc_rabbit_queen0",
            Encounter::RabbitQueen1 => "enc_rabbit_queen1",
            Encounter::GeodeMoth0 => "enc_geode_moth0",
            Encounter::GeodeMoth1 => "enc_geode_moth1",
            Encounter::GeodeMoth2 => "enc_geode_moth2",
            Encounter::GeodeFirefly0 => "enc_geode_firefly0",
            Encounter::GeodeFirefly1 => "enc_geode_firefly1",
            Encounter::GeodeFirefly2 => "enc_geode_firefly2",
            Encounter::GeodeButterfly0 => "enc_geode_butterfly0",
            Encounter::GeodeButterfly1 => "enc_geode_butterfly1",
            Encounter::GeodeButterfly2 => "enc_geode_butterfly2",
            Encounter::DepthsBasilisk0 => "enc_depths_basilisk0",
            Encounter::DepthsBasilisk1 => "enc_depths_basilisk1",
            Encounter::DepthsBeast0 => "enc_depths_beast0",
            Encounter::DepthsBeast1 => "enc_depths_beast1",
            Encounter::DepthsAngel0 => "enc_depths_angel0",
            Encounter::DepthsHound0 => "enc_depths_hound0",
            Encounter::DepthsHound1 => "enc_depths_hound1",
            Encounter::SanctSaph0 => "enc_sanct_saph0",
            Encounter::SanctSaph1 => "enc_sanct_saph1",
            Encounter::SanctCapricorn0 => "enc_sanct_capricorn0",
            Encounter::SanctCapricorn1 => "enc_sanct_capricorn1",
            Encounter::SanctFlower0 => "enc_sanct_flower0",
            Encounter::SanctOwl0 => "enc_sanct_owl0",
            Encounter::SanctOwl1 => "enc_sanct_owl1",
            Encounter::AurumWhitecat0 => "enc_aurum_whitecat0",
            Encounter::AurumWhitecat1 => "enc_aurum_whitecat1",
            Encounter::AurumBeast0 => "enc_aurum_beast0",
            Encounter::AurumBeast1 => "enc_aurum_beast1",
            Encounter::AurumGhost0 => "enc_aurum_ghost0",
            Encounter::AurumBlackcat0 => "enc_aurum_blackcat0",
            Encounter::AurumBlackcat1 => "enc_aurum_blackcat1",
            Encounter::DarkhallSpelllock0 => "enc_darkhall_spelllock0",
            Encounter::DarkhallSpelllock1 => "enc_darkhall_spelllock1",
            Encounter::DarkhallSpelllock2 => "enc_darkhall_spelllock2",
            Encounter::DarkhallSpelllock3 => "enc_darkhall_spelllock3",
            Encounter::DarkhallSpelllock4 => "enc_darkhall_spelllock4",
            Encounter::HeartWitch0 => "enc_heart_witch0",
            Encounter::HeartWitch1 => "enc_heart_witch1",
            Encounter::ToyboxSphere => "enc_toybox_sphere",
            Encounter::ToyboxSphereAttack => "enc_toybox_sphere_attack",
            Encounter::ToyboxSphereMove => "enc_toybox_sphere_move",
        };
        write!(f, "{}", string)
    }
}

impl TryFrom<usize> for Encounter {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Encounter::TrainingNothing),
            1 => Ok(Encounter::TrainingCircles),
            2 => Ok(Encounter::TrainingCircleSpreads),
            3 => Ok(Encounter::TrainingCones),
            4 => Ok(Encounter::TrainingConeSpreads),
            5 => Ok(Encounter::TrainingLine),
            6 => Ok(Encounter::TrainingLineSpreads),
            7 => Ok(Encounter::TrainingPrscircle),
            8 => Ok(Encounter::TrainingLinestack),
            9 => Ok(Encounter::TrainingLinestackFollow),
            10 => Ok(Encounter::TrainingSpinray),
            11 => Ok(Encounter::TrainingSpinfast),
            12 => Ok(Encounter::TrainingEnlargeBullet),
            13 => Ok(Encounter::TrainingEnlargeRay),
            14 => Ok(Encounter::TrainingColormatch),
            15 => Ok(Encounter::TrainingClockspot),
            16 => Ok(Encounter::TrainingCardinal),
            17 => Ok(Encounter::TrainingCleave),
            18 => Ok(Encounter::TrainingTether),
            19 => Ok(Encounter::TrainingTetherFixed),
            20 => Ok(Encounter::TrainingMovecheck),
            21 => Ok(Encounter::TrainingThorns),
            22 => Ok(Encounter::TrainingThornsFixed),
            23 => Ok(Encounter::TrainingMarching),
            24 => Ok(Encounter::TrainingBind),
            25 => Ok(Encounter::TrainingGravity),
            26 => Ok(Encounter::TrainingHeavy),
            27 => Ok(Encounter::TrainingTailwind),
            28 => Ok(Encounter::TrainingTeleport),
            29 => Ok(Encounter::TrainingTimestop),
            30 => Ok(Encounter::TrainingFieldlimit),
            31 => Ok(Encounter::TrainingFieldlimit2),
            32 => Ok(Encounter::TrainingDisplayorder),
            33 => Ok(Encounter::TrainingEnrage),
            34 => Ok(Encounter::TrainingTransform),
            35 => Ok(Encounter::TrainingTransformMinor),
            36 => Ok(Encounter::TrainingDark0),
            37 => Ok(Encounter::TrainingDark1),
            38 => Ok(Encounter::TrainingDark2),
            39 => Ok(Encounter::TrainingSteel0),
            40 => Ok(Encounter::TrainingSteel1),
            41 => Ok(Encounter::TrainingFire0),
            42 => Ok(Encounter::TrainingFire1),
            43 => Ok(Encounter::TrainingLight0),
            44 => Ok(Encounter::TrainingLight1),
            45 => Ok(Encounter::TrainingWater0),
            46 => Ok(Encounter::TrainingWater1),
            47 => Ok(Encounter::TrainingSpinningCardinal),
            48 => Ok(Encounter::TrainingAngelCircle),
            49 => Ok(Encounter::TrainingDeathwall),
            50 => Ok(Encounter::TrainingTargetTether),
            51 => Ok(Encounter::TrainingSummonButterfly),
            52 => Ok(Encounter::TrainingSummonFishpool),
            53 => Ok(Encounter::TrainingSummonJellycat),
            54 => Ok(Encounter::TrainingBloomRing),
            55 => Ok(Encounter::TrainingBlackwhiteSpawn),
            56 => Ok(Encounter::TrainingPlayermirror),
            57 => Ok(Encounter::TrainingSummonAurumsword),
            58 => Ok(Encounter::TrainingGemPick),
            59 => Ok(Encounter::TrainingAngelAppear),
            60 => Ok(Encounter::TrainingPatternErase),
            61 => Ok(Encounter::BirdSophomore0),
            62 => Ok(Encounter::BirdSophomore1),
            63 => Ok(Encounter::BirdSophomore2),
            64 => Ok(Encounter::WolfBlackear0),
            65 => Ok(Encounter::WolfBlackear1),
            66 => Ok(Encounter::WolfBlackear2),
            67 => Ok(Encounter::DragonGranite0),
            68 => Ok(Encounter::DragonGranite1),
            69 => Ok(Encounter::DragonGranite2),
            70 => Ok(Encounter::MouseCadet0),
            71 => Ok(Encounter::MouseCadet1),
            72 => Ok(Encounter::MouseCadet2),
            73 => Ok(Encounter::FrogTinkerer0),
            74 => Ok(Encounter::FrogTinkerer1),
            75 => Ok(Encounter::FrogTinkerer2),
            76 => Ok(Encounter::BirdStudent0),
            77 => Ok(Encounter::BirdStudent1),
            78 => Ok(Encounter::BirdWhispering0),
            79 => Ok(Encounter::BirdWhispering1),
            80 => Ok(Encounter::BirdArchon0),
            81 => Ok(Encounter::BirdValedictorian0),
            82 => Ok(Encounter::BirdValedictorian1),
            83 => Ok(Encounter::WolfGreyeye0),
            84 => Ok(Encounter::WolfGreyeye1),
            85 => Ok(Encounter::WolfBluepaw0),
            86 => Ok(Encounter::WolfBluepaw1),
            87 => Ok(Encounter::WolfSnowfur0),
            88 => Ok(Encounter::WolfSteeltooth0),
            89 => Ok(Encounter::WolfSteeltooth1),
            90 => Ok(Encounter::DragonGold0),
            91 => Ok(Encounter::DragonGold1),
            92 => Ok(Encounter::DragonEmerald0),
            93 => Ok(Encounter::DragonEmerald1),
            94 => Ok(Encounter::DragonRuby0),
            95 => Ok(Encounter::DragonMythril0),
            96 => Ok(Encounter::DragonMythril1),
            97 => Ok(Encounter::MouseArcher0),
            98 => Ok(Encounter::MouseArcher1),
            99 => Ok(Encounter::MouseOakspear0),
            100 => Ok(Encounter::MouseOakspear1),
            101 => Ok(Encounter::MouseRosemage0),
            102 => Ok(Encounter::MousePaladin0),
            103 => Ok(Encounter::MousePaladin1),
            104 => Ok(Encounter::FrogSeamstress0),
            105 => Ok(Encounter::FrogSeamstress1),
            106 => Ok(Encounter::FrogSongstress0),
            107 => Ok(Encounter::FrogSongstress1),
            108 => Ok(Encounter::FrogPainter0),
            109 => Ok(Encounter::FrogIdol0),
            110 => Ok(Encounter::FrogIdol1),
            111 => Ok(Encounter::QueensStaff0),
            112 => Ok(Encounter::QueensKnife0),
            113 => Ok(Encounter::QueensAxe0),
            114 => Ok(Encounter::QueensSpear0),
            115 => Ok(Encounter::QueensHarp0),
            116 => Ok(Encounter::RabbitQueen0),
            117 => Ok(Encounter::RabbitQueen1),
            118 => Ok(Encounter::GeodeMoth0),
            119 => Ok(Encounter::GeodeMoth1),
            120 => Ok(Encounter::GeodeMoth2),
            121 => Ok(Encounter::GeodeFirefly0),
            122 => Ok(Encounter::GeodeFirefly1),
            123 => Ok(Encounter::GeodeFirefly2),
            124 => Ok(Encounter::GeodeButterfly0),
            125 => Ok(Encounter::GeodeButterfly1),
            126 => Ok(Encounter::GeodeButterfly2),
            127 => Ok(Encounter::DepthsBasilisk0),
            128 => Ok(Encounter::DepthsBasilisk1),
            129 => Ok(Encounter::DepthsBeast0),
            130 => Ok(Encounter::DepthsBeast1),
            131 => Ok(Encounter::DepthsAngel0),
            132 => Ok(Encounter::DepthsHound0),
            133 => Ok(Encounter::DepthsHound1),
            134 => Ok(Encounter::SanctSaph0),
            135 => Ok(Encounter::SanctSaph1),
            136 => Ok(Encounter::SanctCapricorn0),
            137 => Ok(Encounter::SanctCapricorn1),
            138 => Ok(Encounter::SanctFlower0),
            139 => Ok(Encounter::SanctOwl0),
            140 => Ok(Encounter::SanctOwl1),
            141 => Ok(Encounter::AurumWhitecat0),
            142 => Ok(Encounter::AurumWhitecat1),
            143 => Ok(Encounter::AurumBeast0),
            144 => Ok(Encounter::AurumBeast1),
            145 => Ok(Encounter::AurumGhost0),
            146 => Ok(Encounter::AurumBlackcat0),
            147 => Ok(Encounter::AurumBlackcat1),
            148 => Ok(Encounter::DarkhallSpelllock0),
            149 => Ok(Encounter::DarkhallSpelllock1),
            150 => Ok(Encounter::DarkhallSpelllock2),
            151 => Ok(Encounter::DarkhallSpelllock3),
            152 => Ok(Encounter::DarkhallSpelllock4),
            153 => Ok(Encounter::HeartWitch0),
            154 => Ok(Encounter::HeartWitch1),
            155 => Ok(Encounter::ToyboxSphere),
            156 => Ok(Encounter::ToyboxSphereAttack),
            157 => Ok(Encounter::ToyboxSphereMove),
            _ => Err("Got a bad Encounter ID"),
        }
    }
}
