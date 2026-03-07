use binrw::binrw;
use derive_debug::Dbg;

use crate::SectionHeader;

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Dbg, Clone)]
#[binrw]
#[br(assert(header.version == Self::VERSION, "info section: invalid version => must be {}", Self::VERSION))]
#[brw(magic = b"INFO", big)]
pub struct InfoHolder {
    header: SectionHeader,
    room_key: u32,
    pub probabilities: [u32; 2],
    pub region: u32,
    pub repick_prevention: u32,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub allow_trophies: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub allow_200cc: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub allow_umt: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub allow_feather: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub allow_mega_cloud: bool,
    pub cup_amount: u16,
    pub online_vote_timer: u8,
    #[dbg(skip)]
    padding: [u8; 40],
}

impl InfoHolder {
    // const MAGIC: &[u8; 4] = b"INFO";
    const VERSION: u32 = 1;
    const SIZE: u32 = 0x50;
}

impl Default for InfoHolder {
    fn default() -> Self {
        Self {
            header: SectionHeader {
                // magic: *Self::MAGIC,
                version: Self::VERSION,
                size: Self::SIZE,
            },
            room_key: Default::default(),
            probabilities: Default::default(),
            region: Default::default(),
            repick_prevention: Default::default(),
            allow_trophies: Default::default(),
            allow_200cc: Default::default(),
            allow_umt: Default::default(),
            allow_feather: Default::default(),
            allow_mega_cloud: Default::default(),
            cup_amount: Default::default(),
            online_vote_timer: Default::default(),
            padding: [0; 40],
        }
    }
}
