use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ChestColor {
    WHITE = 2,
    OPAL = 3,
    SAPPHIRE = 4,
    RUBY = 5,
    GARNET = 6,
    EMERALD = 7,
}

#[derive(Debug, Clone)]
pub struct Chest {
    pub color: ChestColor,
    pub items: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum GemType {
    OPAL,
    SAPPHIRE,
    RUBY,
    GARNET,
    EMERALD,
}

#[derive(Debug, Clone, Copy)]
pub struct Gem {
    pub gem_id: usize,
    pub gem_type: GemType,
    pub price: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Potion {
    pub potion_id: usize,
    pub price: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Shop {
    pub gems: [Gem; 4],
    pub potions: [Potion; 3],
}

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
        if !(504 <= gem_id && gem_id <= 523) {
            return Err("Gem should have ID between 504 and 523");
        }
        if !(23 <= price && price <= 27) {
            return Err("Gem price should be between 23 and 27");
        }
        // Assumes gem upgrade ID's are ordered as Opal (primary) Opal (secondary) Opal Opal, Sapphire ...
        let gem_type = GemType::try_from((gem_id - 504) / 4)?;
        Ok(Self {
            gem_id,
            gem_type,
            price,
        })
    }
}

impl Potion {
    pub fn from_id_price(potion_id: usize, price: usize) -> Result<Self, &'static str> {
        if !(489 <= potion_id && potion_id <= 503) {
            return Err("Potion should have ID between 489 and 503");
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

        bitmask
    }
}
