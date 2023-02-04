use byteorder::{LittleEndian, WriteBytesExt};

pub struct Song {
    pub start: u64,
    pub index: usize,
    pub real_size: usize,
    // As calculated; padded out to be divisible by 16
    pub size: usize,
}

impl Song {
    pub fn as_header(&self) -> Vec<u8> {
        let mut out = vec![];
        out.write_u32::<LittleEndian>(self.start as u32).unwrap();
        out.write_u32::<LittleEndian>(self.real_size as u32)
            .unwrap();

        out
    }
}
