use std::fmt::{Debug, Display};

use binrw::binrw;

macro_rules! define_ids {
    ($name:ident, $all_size:literal, $default:ident, {
        $($label:ident = $value:expr => $display:literal),* $(,)?
    }) => {
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        #[binrw]
        #[brw(big, repr = u8)]
        pub enum $name {
            $($label = $value,)*
        }

        impl $name {
            pub const ALL: [Self; $all_size] = [
                $(Self::$label,)*
            ];
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$label => write!(f, $display),)*
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$label => write!(f, $display),)*
                }
            }
        }
    };
}

define_ids!(CourseId, 42, MarioCircuit, {
    LuigiCircuit = 0x08 => "Luigi Circuit",
    MooMooMeadows = 0x01 => "Moo Moo Meadows",
    MushroomGorge = 0x02 => "Mushroom Gorge",
    ToadsFactory = 0x04 => "Toad's Factory",

    MarioCircuit = 0x00 => "Mario Circuit",
    CoconutMall = 0x05 => "Coconut Mall",
    DkSummit = 0x06 => "DK Summit",
    WariosGoldMine = 0x07 => "Wario's Gold Mine",

    DaisyCircuit = 0x09 => "Daisy Circuit",
    KoopaCape = 0x0F => "Koopa Cape",
    MapleTreeway = 0x0B => "Maple Treeway",
    GrumbleVolcano = 0x03 => "Grumble Volcano",

    DryDryRuins = 0x0E => "Dry Dry Ruins",
    MoonviewHighway = 0x0A => "Moonview Highway",
    BowsersCastle = 0x0C => "Bowser's Castle",
    RainbowRoad = 0x0D => "Rainbow Road",

    GcnPeachBeach = 0x10 => "GCN Peach Beach",
    DsYoshiFalls = 0x14 => "DS Yoshi Falls",
    SnesGhostValley2 = 0x19 => "SNES Ghost Valley 2",
    N64MarioRaceway = 0x1A => "N64 Mario Raceway",

    N64SherbetLand = 0x1B => "N64 Sherbet Land",
    GbaShyGuyBeach = 0x1F => "GBA Shy Guy Beach",
    DsDelfinoSquare = 0x17 => "DS Delfino Square",
    GcnWaluigiStadium = 0x12 => "GCN Waluigi Stadium",

    DsDesertHills = 0x15 => "DS Desert Hills",
    GbaBowserCastle3 = 0x1E => "GBA Bowser Castle 3",
    N64DkJungleParkway = 0x1D => "N64 DK Jungle Parkway",
    GcnMarioCircuit = 0x11 => "GCN Mario Circuit",

    SnesMarioCircuit3 = 0x18 => "SNES Mario Circuit 3",
    DsPeachGardens = 0x16 => "DS Peach Gardens",
    GcnDkMountain = 0x13 => "GCN DK Mountain",
    N64BowsersCastle = 0x1C => "N64 Bowsers Castle",

    BlockPlaza = 0x21 => "Block Plaza",
    DelfinoPier = 0x20 => "Delfino Pier",
    FunkyStadium = 0x23 => "Funky Stadium",
    ChainChompWheel = 0x22 => "Chain Chomp Wheel",
    ThwompDesert = 0x24 => "Thwomp Desert",

    SnesBattleCourse4 = 0x27 => "SNES Battle Course 4",
    GbaBattleCourse3 = 0x28 => "GBA Battle Course 3",
    N64Skyscraper = 0x29 => "N64 Skyscraper",
    GcnCookieLand = 0x25 => "GCN Cookie Land",
    DsTwilightHouse = 0x26 => "DS Twilight House",
});

// Music Slot ID translates in engine
define_ids!(MusicId, 43, MarioCircuit, {
    LuigiCircuit = 0x08 => "Luigi Circuit",
    MooMooMeadows = 0x01 => "Moo Moo Meadows",
    MushroomGorge = 0x02 => "Mushroom Gorge",
    ToadsFactory = 0x04 => "Toad's Factory",

    MarioCircuit = 0x00 => "Mario Circuit",
    CoconutMall = 0x05 => "Coconut Mall",
    DkSummit = 0x06 => "DK Summit",
    WariosGoldMine = 0x07 => "Wario's Gold Mine",

    DaisyCircuit = 0x09 => "Daisy Circuit",
    KoopaCape = 0x0F => "Koopa Cape",
    MapleTreeway = 0x0B => "Maple Treeway",
    GrumbleVolcano = 0x03 => "Grumble Volcano",

    DryDryRuins = 0x0E => "Dry Dry Ruins",
    MoonviewHighway = 0x0A => "Moonview Highway",
    BowsersCastle = 0x0C => "Bowser's Castle",
    RainbowRoad = 0x0D => "Rainbow Road",

    GcnPeachBeach = 0x10 => "GCN Peach Beach",
    DsYoshiFalls = 0x14 => "DS Yoshi Falls",
    SnesGhostValley2 = 0x19 => "SNES Ghost Valley 2",
    N64MarioRaceway = 0x1A => "N64 Mario Raceway",

    N64SherbetLand = 0x1B => "N64 Sherbet Land",
    GbaShyGuyBeach = 0x1F => "GBA Shy Guy Beach",
    DsDelfinoSquare = 0x17 => "DS Delfino Square",
    GcnWaluigiStadium = 0x12 => "GCN Waluigi Stadium",

    DsDesertHills = 0x15 => "DS Desert Hills",
    GbaBowserCastle3 = 0x1E => "GBA Bowser Castle 3",
    N64DkJungleParkway = 0x1D => "N64 DK Jungle Parkway",
    GcnMarioCircuit = 0x11 => "GCN Mario Circuit",

    SnesMarioCircuit3 = 0x18 => "SNES Mario Circuit 3",
    DsPeachGardens = 0x16 => "DS Peach Gardens",
    GcnDkMountain = 0x13 => "GCN DK Mountain",
    N64BowsersCastle = 0x1C => "N64 Bowsers Castle",

    BlockPlaza = 0x21 => "Block Plaza",
    DelfinoPier = 0x20 => "Delfino Pier",
    FunkyStadium = 0x23 => "Funky Stadium",
    ChainChompWheel = 0x22 => "Chain Chomp Wheel",
    ThwompDesert = 0x24 => "Thwomp Desert",

    SnesBattleCourse4 = 0x27 => "SNES Battle Course 4",
    GbaBattleCourse3 = 0x28 => "GBA Battle Course 3",
    N64Skyscraper = 0x29 => "N64 Skyscraper",
    GcnCookieLand = 0x25 => "GCN Cookie Land",
    DsTwilightHouse = 0x26 => "DS Twilight House",

    GalaxyColosseum = 0x36 => "Galaxy Colosseum",
});
