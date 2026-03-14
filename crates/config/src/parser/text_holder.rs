use binrw::{BinWrite as _, binrw};
use std::{fmt::Debug, fs, io::{Cursor, Write as _}, path::Path};

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
    pub fn as_cursor(&self) -> Cursor<Vec<u8>> {
        let mut temp: Vec<u8> = b"MESGbmg1".to_vec();
        temp.extend_from_slice(&self.size.to_be_bytes());
        temp.extend_from_slice(&self.bytes);
        Cursor::new(temp)
    }

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
