use std::fs::File;
use std::io::Cursor;
use std::io::Error;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

pub const TRACK_COUNT: usize = 40;
pub const TRACK_HEADER_LENGTH: usize = 8;

pub struct Song {
    pub start: u32,
    pub index: usize,
}

fn main() -> Result<(), Error> {
    let mut file = File::open("Stream.bin")?;
    let mut buf: Vec<u8> = vec![0; TRACK_COUNT * TRACK_HEADER_LENGTH];
    file.read_exact(&mut buf)?;

    let mut songs = vec![];

    for i in 1..TRACK_COUNT {
        let index = (i - 1) * TRACK_HEADER_LENGTH;

        let mut reader = Cursor::new(&buf);
        reader.set_position(index as u64);

        let start = reader.read_u32::<LittleEndian>().unwrap();

        songs.push(Song { start, index });
    }

    for song in songs {
        println!("Song {} starts at {}", song.index, song.start);
    }

    Ok(())
}
