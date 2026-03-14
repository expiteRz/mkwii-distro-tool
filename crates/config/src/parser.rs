use std::{fs, io::{Read, Seek}, path::Path};

use binrw::{BinRead as _, binrw};
use mkwii_distro_tool_bmg::parse::BMG;

use crate::{parser::{binary_header::BinaryHeader, cup_holder::CupHolder, info_holder::InfoHolder, text_holder::TextHolder}, traits::ParseExt};

pub mod binary_header;
pub mod cup_holder;
pub mod info_holder;
pub mod text_holder;

#[rustfmt::skip]
#[derive(Debug, Default, Clone)]
pub struct RawConfig {
    pub header: BinaryHeader,
    pub info: InfoHolder,
    pub cups: CupHolder,
    pub texts: BMG, // BMG // TODO: Verify bytes is actual BMG
    // TODO: Analyze how Creator reads cup and track names from Config.pul
}

impl RawConfig {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut bytes = fs::File::open(path)?;
        Self::read(&mut bytes)
    }
}

impl ParseExt for RawConfig {
    fn read<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        let header = BinaryHeader::read(reader)?;
        let info = InfoHolder::read(reader)?;
        let cups = CupHolder::read(reader)?;
        let mut _texts = TextHolder::read(reader)?;
        let texts = BMG::read(&mut _texts.as_cursor())?;

        Ok(Self { header, info, cups, texts })
    }
}

// #[repr(C)]
#[derive(Debug, Clone, Default)]
#[binrw]
pub struct SectionHeader {
    // magic: [u8; 4],
    version: u32,
    size: u32,
}

#[allow(unused_must_use)]
#[cfg(test)]
mod test {
    use std::{fs, io::Write as _, path::Path};

    use binrw::BinWrite as _;

    use crate::{parser::{RawConfig, info_holder::InfoHolder}, traits::ParseExt as _};

    #[test]
    fn test_write_info() {
        let path = Path::new("test");
        if !path.exists() {
            fs::create_dir_all(path);
        }
        let path = path.join("info.bin");
        let mut file = fs::File::create(path).expect("expected at creating a file");
        let info = InfoHolder::default();
        info.write_be(&mut file);
        file.flush();
    }

    #[test]
    fn test_read_all() {
        let path = Path::new("test/Config.pul");
        let mut file = fs::File::open(path).expect("expected at opening Config.pul");
        let config = RawConfig::read(&mut file).expect("expected at reading Config.pul");
        file.flush();
        println!("{config:?}");
        println!(
            "Cup section size: Binary: {}, Rust calculated: {}",
            config.cups.header.size,
            config.cups.size()
        );
    }
}

