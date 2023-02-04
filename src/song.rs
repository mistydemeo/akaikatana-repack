use byteorder::{LittleEndian, WriteBytesExt};

pub struct Song {
    pub start: u64,
    pub index: usize,
    pub size: usize,
}

impl Song {
    pub fn as_header(&self) -> Vec<u8> {
        let mut out = vec![];
        out.write_u32::<LittleEndian>(self.start as u32).unwrap();

        out
    }
}
