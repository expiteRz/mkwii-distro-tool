use std::{fmt::Debug, fs, io::{Read, Seek, Write}, path::Path};

use binrw::{BinRead, BinWrite, binrw};
use iced::{Element, Task, widget::column};
use mkwii_distro_tool_ui::{self as ui, View as _};

use crate::{binary_header::BinaryHeader, cup_holder::CupHolder, info_holder::InfoHolder, traits::ParseExt};

pub mod binary_header;
pub mod cup_holder;
pub mod info_holder;
pub mod slots;
pub mod traits;

#[derive(Clone)]
pub enum ParentMessage {
    BinaryHeader(binary_header::Message),
    InfoHolder(info_holder::Message),
    CupHolder(cup_holder::Message),
    Error(String),
}

#[rustfmt::skip]
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub header: BinaryHeader,
    pub info: InfoHolder,
    pub cups: CupHolder,
    texts: TextHolder, // BMG // TODO: Verify bytes is actual BMG
    // TODO: Analyze how Creator reads cup and track names from Config.pul
}

impl Config {
    pub fn new() -> (Self, Task<ParentMessage>) {
        (
            Self {
                header: BinaryHeader::default(),
                info: InfoHolder::default(),
                cups: CupHolder::default(),
                texts: TextHolder::default(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: ParentMessage) {
        match message {
            ParentMessage::BinaryHeader(message) => {
                self.header.update(message);
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, ParentMessage> {
        let header = self.header.view().map(ParentMessage::BinaryHeader);
        let cups = self.cups.view().map(ParentMessage::CupHolder);
        column![header, cups].into()
    }

    pub fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut bytes = fs::File::open(path)?;
        Self::read(&mut bytes)
    }
}

impl ParseExt for Config {
    fn read<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        let header = BinaryHeader::read(reader)?;
        let info = InfoHolder::read(reader)?;
        let cups = CupHolder::read(reader)?;
        let texts = TextHolder::read(reader)?;

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

#[derive(Clone, Default)]
#[binrw]
#[brw(magic = b"MESGbmg1", big)]
pub struct TextHolder {
    size: u32,
    #[br(count = size - 12)]
    bytes: Vec<u8>,
}

impl Debug for TextHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextHolder").field("size", &self.size).field("bytes", &"...").finish()
    }
}

impl TextHolder {
    pub fn write_temporary(&self) -> anyhow::Result<()> {
        let temp_dir_path = Path::new("temp");
        if !temp_dir_path.exists() {
            fs::create_dir_all(temp_dir_path)?;
        }

        let dest_path = temp_dir_path.join("Pul.bmg");
        let mut file = fs::File::create(dest_path)?;
        self.write(&mut file)?;
        file.flush()?;

        Ok(())
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod test {
    use std::{fs, io::Write as _, path::Path};

    use binrw::BinWrite as _;

    use crate::{Config, InfoHolder, traits::ParseExt as _};

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
        let config = Config::read(&mut file).expect("expected at reading Config.pul");
        file.flush();
        println!("{config:?}");
        println!(
            "Cup section size: Binary: {}, Rust calculated: {}",
            config.cups.header.size,
            config.cups.size()
        );
    }
}
