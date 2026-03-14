use std::io::{Read, Seek};

use anyhow::anyhow;

use crate::traits::{ParseExt, ReadArrayExt as _};

// #[repr(C)]
#[derive(Debug, Clone)]
pub struct BinaryHeader {
    magic: [u8; 4],
    version: i32,      // TODO: Needs support older version and newer version (if available)
    offsets: [u32; 3], // INFO, CUPS, BMG
    pub dir_name: String,
}

impl BinaryHeader {
    const MAGIC: &[u8; 4] = b"PULS";
    const VERSION: i32 = 3;
    const INFO_HOLDER_OFFSET: u32 = 0x24;
    const CUP_HOLDER_OFFSET: u32 = 0x74;

    /// Removes slash on the first character and nulls
    fn fix_dir_name(name: String) -> String {
        let mut dir_name: String = name.chars().skip(1).collect();
        dir_name = dir_name.trim_matches(char::from(0)).to_owned();
        dir_name
    }
}

impl ParseExt for BinaryHeader {
    fn read<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        let magic: [u8; 4] = reader.read_array()?;
        if magic != *Self::MAGIC {
            return Err(anyhow!("expected at reading Config.pul: invalid magic => must be 'PULS'"));
        }
        let version: i32 = reader.read_number()?;
        if version != Self::VERSION {
            return Err(anyhow!("expected at reading Config.pul: invalid version => must be {}", Self::VERSION));
        }
        let offsets: [u32; 3] = reader.read_array()?;
        if offsets[0] != Self::INFO_HOLDER_OFFSET && offsets[1] != Self::CUP_HOLDER_OFFSET {
            return Err(anyhow!("expected at reading Config.pul: invalid offsets"));
        }
        let _dir_name: [u8; 16] = reader.read_array()?;
        let mut dir_name = String::from_utf8(_dir_name.to_vec())?;
        dir_name = Self::fix_dir_name(dir_name);

        Ok(Self {
            magic,
            version,
            offsets,
            dir_name,
        })
    }
}

impl Default for BinaryHeader {
    fn default() -> Self {
        Self {
            magic: *Self::MAGIC,
            version: Self::VERSION,
            offsets: [Self::INFO_HOLDER_OFFSET, Self::CUP_HOLDER_OFFSET, 0],
            dir_name: Default::default(),
        }
    }
}
