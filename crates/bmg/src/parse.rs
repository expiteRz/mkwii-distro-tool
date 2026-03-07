// Parse the original BMG file

use std::{fmt::Debug, fs, path::Path};

use binrw::{BinRead as _, binrw, helpers::until_eof};
use serde::{Deserialize, Serialize};

use crate::{MessageGroup, MessageItem, get_text_until_null, traits::ExistSubstringExt};

#[derive(Debug, Default, Clone)]
#[binrw]
#[brw(magic = b"MESGbmg1")]
pub struct BMG {
    header: BMGHeader,
    pub entry_holder: EntryHolder,
    pub pool_holder: StringHolder,
    pub ids_holder: MessageIdHolder,
}

impl BMG {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut file = fs::File::open(path)?;
        let data = Self::read_be(&mut file)?;
        Ok(data)
    }
}

impl Into<MessageGroup> for BMG {
    fn into(self) -> MessageGroup {
        let mut items: Vec<MessageItem> = vec![];

        let ids = &self.ids_holder.ids;
        let entry = &self.entry_holder;
        let pool = &self.pool_holder.data;

        for i in 0..entry.message_len as usize {
            let item = MessageItem {
                id: ids[i],
                font_type: entry.entries[i].font_type.clone(),
                text: get_text_until_null(pool, (entry.entries[i].offset / 2) as usize),
            };
            items.push(item);
        }

        MessageGroup { items }
    }
}

impl From<MessageGroup> for BMG {
    fn from(value: MessageGroup) -> Self {
        let items = value.items;

        let mut entries: Vec<TextEntry> = vec![];
        let mut text_pool: Vec<u16> = vec![0x0000]; // Start by null
        let mut ids: Vec<u32> = vec![];

        // Text assigning
        for mut item in items {
            item.text.push_str("\0");
            let text: Vec<u16> = item.text.encode_utf16().collect();
            let mut offset = if text.len() != 0 { text_pool.len() * 2 } else { 0 };
            if text.len() != 0
                && let Some(i) = text_pool.check_substring_exist(&text)
            {
                offset = i * 2;
            }
            text_pool.extend_from_slice(&text);
            let entry = TextEntry::new(offset, item.font_type);
            ids.push(item.id);
            entries.push(entry);
        }

        // Entry holder preparation
        let message_len = entries.len() as u16;
        entries.resize(entries.len() + (entries.len() % 2), TextEntry::new_by_zero());
        let entry_holder_size = EntryHolder::HEADER_SIZE + (entries.len() * 8) as u32;
        let mut entry_holder = EntryHolder::default();
        {
            entry_holder.size = entry_holder_size;
            entry_holder.message_len = message_len;
            entry_holder.entries = entries;
        }

        // Text pool preparation
        text_pool.resize(text_pool.len() + (8 - (text_pool.len() % 8)), 0x0u16);
        let pool_holder_size = StringHolder::HEADER_SIZE + (text_pool.len() * 2) as u32;
        let mut pool_holder = StringHolder::default();
        {
            pool_holder.size = pool_holder_size;
            pool_holder.data = text_pool;
        }

        // Message ID holder preparation
        let entry_len = ids.len() as u16;
        ids.resize(ids.len() + (4 - (ids.len() % 4)), 0x0u32);
        let ids_holder_size = MessageIdHolder::HEADER_SIZE + (ids.len() * 4) as u32;
        let mut ids_holder = MessageIdHolder::default();
        {
            ids_holder.size = ids_holder_size;
            ids_holder.entry_len = entry_len;
            ids_holder.ids = ids;
        }

        // Main header preparation
        let mut header = BMGHeader::default();
        header.size = BMGHeader::HEADER_SIZE + entry_holder_size + pool_holder_size + ids_holder_size;

        Self {
            header,
            entry_holder,
            pool_holder,
            ids_holder,
        }
    }
}

#[derive(Debug, Clone)]
#[binrw]
#[brw(big)]
pub struct BMGHeader {
    size: u32,
    section_number: u32,
    charset: Charset,
    padding: [u8; 15],
}

impl BMGHeader {
    const HEADER_SIZE: u32 = 0x20;
}

impl Default for BMGHeader {
    fn default() -> Self {
        Self {
            size: Default::default(),
            section_number: 3,
            charset: Default::default(),
            padding: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
#[binrw]
#[brw(repr = u8, big)]
pub enum Charset {
    None = 0,
    OneByte,
    #[default]
    TwoBytes, // UTF16; Default for MKWii
    ShiftJIS,
    UTF8,
}

#[derive(Debug, Clone)]
#[binrw]
#[brw(magic = b"INF1", big)]
pub struct EntryHolder {
    size: u32,
    pub message_len: u16,
    entry_size: u16, // always 8 in mkwii
    group_id: u16,
    default_color: u8,
    padding: u8,
    #[br(count = message_len + (message_len % 4))]
    pub entries: Vec<TextEntry>,
}

impl EntryHolder {
    const HEADER_SIZE: u32 = 0x10;
}

impl Default for EntryHolder {
    fn default() -> Self {
        Self {
            size: Default::default(),
            message_len: Default::default(),
            entry_size: 8,
            group_id: Default::default(),
            default_color: Default::default(),
            padding: Default::default(),
            entries: vec![],
        }
    }
}

#[derive(Default, Clone)]
#[binrw]
#[brw(big)]
pub struct TextEntry {
    pub offset: u32,
    pub font_type: FontType,
    padding: [u8; 3],
}

impl TextEntry {
    pub fn new(offset: usize, font_type: FontType) -> Self {
        Self {
            offset: offset as u32,
            font_type,
            ..Default::default()
        }
    }

    pub fn new_by_zero() -> Self {
        Self {
            offset: 0,
            font_type: FontType::InRace, // meant 0
            ..Default::default()
        }
    }
}

impl Debug for TextEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("offset", &self.offset)
            .field("font_type", &self.font_type)
            .finish()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[binrw]
#[brw(repr = u8, big)]
pub enum FontType {
    InRace = 0,
    #[default]
    General,
    Extension, // unused
    Indicator, // unused
    TeamRed,
    TeamBlue,
}

#[derive(Default, Clone)]
#[binrw]
#[brw(magic = b"DAT1", big)]
pub struct StringHolder {
    size: u32,
    #[br(count = (size - 8) / 2)]
    pub data: Vec<u16>,
}

impl StringHolder {
    const HEADER_SIZE: u32 = 0x8;
}

impl Debug for StringHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringPool")
            .field("size", &self.size)
            .field("data", &format_args!("\"{}\"", String::from_utf16_lossy(&self.data)))
            .finish()
    }
}

#[derive(Clone)]
#[binrw]
#[brw(magic = b"MID1", big)]
pub struct MessageIdHolder {
    size: u32,
    entry_len: u16,
    format: u8,
    info: u8,
    padding: u32,
    // #[br(count = entry_len + (entry_len % 4))]
    #[br(parse_with = until_eof)]
    pub ids: Vec<u32>,
}

impl Default for MessageIdHolder {
    fn default() -> Self {
        Self {
            size: Default::default(),
            entry_len: Default::default(),
            format: 0x10,
            info: Default::default(),
            padding: Default::default(),
            ids: Default::default(),
        }
    }
}

impl MessageIdHolder {
    const HEADER_SIZE: u32 = 0x10;
}

impl Debug for MessageIdHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ids: Vec<String> = self.ids.iter().map(|v| format!("{:X}", *v)).collect();
        f.debug_struct("MessageIdHolder")
            .field("size", &self.size)
            .field("entry_len", &self.entry_len)
            .field("format", &self.format)
            .field("info", &self.info)
            .field("padding", &self.padding)
            .field("ids", &ids)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use crate::parse::BMG;

    #[test]
    #[ignore]
    fn read_bmg() -> anyhow::Result<()> {
        let bmg = BMG::read_from_path("test/Common.bmg")?;
        println!("{bmg:?}");
        Ok(())
    }
}
