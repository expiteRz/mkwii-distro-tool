use std::fmt::{Debug, Display};

use binrw::binrw;

// Music Slot ID translates in engine
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[binrw]
#[brw(big, repr = u8)]
pub enum CourseId {
    LuigiCircuit = 0x08,
    MooMooMeadows = 0x01,
    MushroomGorge = 0x02,
    ToadsFactory = 0x04,

    #[default]
    MarioCircuit = 0x00,
    CoconutMall = 0x05,
    DkSummit = 0x06,
    WariosGoldMine = 0x07,

    DaisyCircuit = 0x09,
    KoopaCape = 0x0F,
    MapleTreeway = 0x0B,
    GrumbleVolcano = 0x03,

    DryDryRuins = 0x0E,
    MoonviewHighway = 0x0A,
    BowsersCastle = 0x0C,
    RainbowRoad = 0x0D,

    GcnPeachBeach = 0x10,
    DsYoshiFalls = 0x14,
    SnesGhostValley2 = 0x19,
    N64MarioRaceway = 0x1A,

    N64SherbetLand = 0x1B,
    GbaShyGuyBeach = 0x1F,
    DsDelfinoSquare = 0x17,
    GcnWaluigiStadium = 0x12,

    DsDesertHills = 0x15,
    GbaBowserCastle3 = 0x1E,
    N64DkJungleParkway = 0x1D,
    GcnMarioCircuit = 0x11,

    SnesMarioCircuit3 = 0x18,
    DsPeachGardens = 0x16,
    GcnDkMountain = 0x13,
    N64BowsersCastle = 0x1C,

    BlockPlaza = 0x21,
    DelfinoPier = 0x20,
    FunkyStadium = 0x23,
    ChainChompWheel = 0x22,
    ThwompDesert = 0x24,

    SnesBattleCourse4 = 0x27,
    GbaBattleCourse3 = 0x28,
    N64Skyscraper = 0x29,
    GcnCookieLand = 0x25,
    DsTwilightHouse = 0x26,

    GalaxyColosseum = 0x36,
}

impl Debug for CourseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LuigiCircuit => write!(f, "Luigi Circuit"),
            Self::MooMooMeadows => write!(f, "Moo Moo Meadows"),
            Self::MushroomGorge => write!(f, "Mushroom Gorge"),
            Self::ToadsFactory => write!(f, "Toad's Factory"),

            Self::MarioCircuit => write!(f, "Mario Circuit"),
            Self::CoconutMall => write!(f, "Coconut Mall"),
            Self::DkSummit => write!(f, "DK Summit"),
            Self::WariosGoldMine => write!(f, "Wario's Gold Mine"),

            Self::DaisyCircuit => write!(f, "Daisy Circuit"),
            Self::KoopaCape => write!(f, "Koopa Cape"),
            Self::MapleTreeway => write!(f, "Maple Treeway"),
            Self::GrumbleVolcano => write!(f, "Grumble Volcano"),

            Self::DryDryRuins => write!(f, "Dry Dry Ruins"),
            Self::MoonviewHighway => write!(f, "Moonview Highway"),
            Self::BowsersCastle => write!(f, "Bowser's Castle"),
            Self::RainbowRoad => write!(f, "Rainbow Road"),

            Self::GcnPeachBeach => write!(f, "GCN Peach Beach"),
            Self::DsYoshiFalls => write!(f, "DS Yoshi Falls"),
            Self::SnesGhostValley2 => write!(f, "SNES Ghost Valley 2"),
            Self::N64MarioRaceway => write!(f, "N64 Mario Raceway"),

            Self::N64SherbetLand => write!(f, "N64 Sherbet Land"),
            Self::GbaShyGuyBeach => write!(f, "GBA Shy Guy Beach"),
            Self::DsDelfinoSquare => write!(f, "DS Delfino Square"),
            Self::GcnWaluigiStadium => write!(f, "GCN Waluigi Stadium"),

            Self::DsDesertHills => write!(f, "DS Desert Hills"),
            Self::GbaBowserCastle3 => write!(f, "GBA Bowser Castle 3"),
            Self::N64DkJungleParkway => write!(f, "N64 DK Jungle Parkway"),
            Self::GcnMarioCircuit => write!(f, "GCN Mario Circuit"),

            Self::SnesMarioCircuit3 => write!(f, "SNES Mario Circuit 3"),
            Self::DsPeachGardens => write!(f, "DS Peach Gardens"),
            Self::GcnDkMountain => write!(f, "GCN DK Mountain"),
            Self::N64BowsersCastle => write!(f, "N64 Bowsers Castle"),

            Self::BlockPlaza => write!(f, "Block Plaza"),
            Self::DelfinoPier => write!(f, "Delfino Pier"),
            Self::FunkyStadium => write!(f, "Funky Stadium"),
            Self::ChainChompWheel => write!(f, "Chain Chomp Wheel"),
            Self::ThwompDesert => write!(f, "Thwomp Desert"),

            Self::SnesBattleCourse4 => write!(f, "SNES Battle Course 4"),
            Self::GbaBattleCourse3 => write!(f, "GBA Battle Course 3"),
            Self::N64Skyscraper => write!(f, "N64 Skyscraper"),
            Self::GcnCookieLand => write!(f, "GCN Cookie Land"),
            Self::DsTwilightHouse => write!(f, "DS Twilight House"),

            Self::GalaxyColosseum => write!(f, "Galaxy Colosseum"),
        }
    }
}

impl Display for CourseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LuigiCircuit => write!(f, "Luigi Circuit"),
            Self::MooMooMeadows => write!(f, "Moo Moo Meadows"),
            Self::MushroomGorge => write!(f, "Mushroom Gorge"),
            Self::ToadsFactory => write!(f, "Toad's Factory"),

            Self::MarioCircuit => write!(f, "Mario Circuit"),
            Self::CoconutMall => write!(f, "Coconut Mall"),
            Self::DkSummit => write!(f, "DK Summit"),
            Self::WariosGoldMine => write!(f, "Wario's Gold Mine"),

            Self::DaisyCircuit => write!(f, "Daisy Circuit"),
            Self::KoopaCape => write!(f, "Koopa Cape"),
            Self::MapleTreeway => write!(f, "Maple Treeway"),
            Self::GrumbleVolcano => write!(f, "Grumble Volcano"),

            Self::DryDryRuins => write!(f, "Dry Dry Ruins"),
            Self::MoonviewHighway => write!(f, "Moonview Highway"),
            Self::BowsersCastle => write!(f, "Bowser's Castle"),
            Self::RainbowRoad => write!(f, "Rainbow Road"),

            Self::GcnPeachBeach => write!(f, "GCN Peach Beach"),
            Self::DsYoshiFalls => write!(f, "DS Yoshi Falls"),
            Self::SnesGhostValley2 => write!(f, "SNES Ghost Valley 2"),
            Self::N64MarioRaceway => write!(f, "N64 Mario Raceway"),

            Self::N64SherbetLand => write!(f, "N64 Sherbet Land"),
            Self::GbaShyGuyBeach => write!(f, "GBA Shy Guy Beach"),
            Self::DsDelfinoSquare => write!(f, "DS Delfino Square"),
            Self::GcnWaluigiStadium => write!(f, "GCN Waluigi Stadium"),

            Self::DsDesertHills => write!(f, "DS Desert Hills"),
            Self::GbaBowserCastle3 => write!(f, "GBA Bowser Castle 3"),
            Self::N64DkJungleParkway => write!(f, "N64 DK Jungle Parkway"),
            Self::GcnMarioCircuit => write!(f, "GCN Mario Circuit"),

            Self::SnesMarioCircuit3 => write!(f, "SNES Mario Circuit 3"),
            Self::DsPeachGardens => write!(f, "DS Peach Gardens"),
            Self::GcnDkMountain => write!(f, "GCN DK Mountain"),
            Self::N64BowsersCastle => write!(f, "N64 Bowsers Castle"),

            Self::BlockPlaza => write!(f, "Block Plaza"),
            Self::DelfinoPier => write!(f, "Delfino Pier"),
            Self::FunkyStadium => write!(f, "Funky Stadium"),
            Self::ChainChompWheel => write!(f, "Chain Chomp Wheel"),
            Self::ThwompDesert => write!(f, "Thwomp Desert"),

            Self::SnesBattleCourse4 => write!(f, "SNES Battle Course 4"),
            Self::GbaBattleCourse3 => write!(f, "GBA Battle Course 3"),
            Self::N64Skyscraper => write!(f, "N64 Skyscraper"),
            Self::GcnCookieLand => write!(f, "GCN Cookie Land"),
            Self::DsTwilightHouse => write!(f, "DS Twilight House"),

            Self::GalaxyColosseum => write!(f, "Galaxy Colosseum"),
        }
    }
}
