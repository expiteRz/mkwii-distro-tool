use std::fmt::Debug;

use binrw::binrw;

use crate::{parser::SectionHeader, slots::{CourseId, MusicId}};

#[derive(Clone, Default)]
#[binrw]
#[br(assert(header.version == Self::VERSION, "cup section: invalid version => must be {}", Self::VERSION))]
#[brw(magic = b"CUPS", big)]
pub struct CupHolder {
    pub header: SectionHeader,
    pub cup_amount: u16,
    pub regular_mode: RegularTrackSeletion,
    pub padding: u8, // In mkwii-distro-tool it used to identify the config has unused alphabetical sorted tracks
    pub trophy_count: [u16; 4],
    pub total_variants: u32,
    #[br(count = usize::from(cup_amount * 4))]
    pub tracks: Vec<RawTrack>,
    #[br(count = total_variants)]
    pub variants: Vec<RawVariant>,
    #[br(if(padding == 0))]
    #[br(count = cup_amount * 4)]
    #[bw(if(*padding > 0))]
    track_alphabeticals: Vec<u16>,
}

impl Debug for CupHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CupHolder")
            .field("header", &self.header)
            .field("cup_amount", &self.cup_amount)
            .field("regular_mode", &self.regular_mode)
            .field("padding", &self.padding)
            .field("trophy_count", &self.trophy_count)
            .field("total_variants", &self.total_variants)
            .field("tracks", &self.tracks)
            .field("variants", &self.variants)
            .field("track_alphabeticals", &"...")
            .finish()
    }
}

#[derive(Debug, Clone, Default)]
#[binrw]
#[brw(repr = u8)]
pub enum RegularTrackSeletion {
    None = 0,
    #[default]
    FirstCups,
    LastCups,
}

#[derive(Debug, Clone, Default)]
#[binrw]
#[brw(big)]
pub struct RawTrack {
    pub slot: CourseId,
    pub music: MusicId,
    pub variant_count: u16,
    crc32: u32,
}

#[derive(Debug, Clone, Default)]
#[binrw]
#[brw(big)]
pub struct RawVariant {
    pub slot: CourseId,
    pub music: MusicId,
}

impl CupHolder {
    // const MAGIC: &[u8; 4] = b"CUPS";
    const VERSION: u32 = 3;
    const SIZE_PRESERVED: u32 = 0x1C;
    const SIZE_PER_TRACK: u32 = 8;
    const SIZE_PER_VARIANT: u32 = 2;
    const SIZE_PER_TRACK_ALPHABETICAL_SORT: u32 = 2; // Same as variant

    /// Calculate size of cup section when build
    pub fn size(&self) -> u32 {
        let track_len = ((self.cup_amount * 4) as u32) * Self::SIZE_PER_TRACK;
        let variant_len = self.total_variants * Self::SIZE_PER_VARIANT;
        let alphabeticals_len = (self.track_alphabeticals.len() as u32) * Self::SIZE_PER_TRACK_ALPHABETICAL_SORT; // needed?

        Self::SIZE_PRESERVED + track_len + variant_len + alphabeticals_len
    }
}
