use std::fs;
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
    pub size: usize,
}

fn main() -> Result<(), Error> {
    let mut file = File::open("Stream.bin")?;
    let mut buf: Vec<u8> = vec![0; TRACK_COUNT * TRACK_HEADER_LENGTH];
    file.read_exact(&mut buf)?;

    let mut songs: Vec<Song> = vec![];

    for i in 1..TRACK_COUNT {
        let index = (i - 1) * TRACK_HEADER_LENGTH;

        let mut reader = Cursor::new(&buf);
        reader.set_position(index as u64);

        let start = reader.read_u32::<LittleEndian>().unwrap();

        // For every song  other than the last, we can calculate the
        // duration based on the current index and the last song
        if i > 1 {
            songs[i - 2].size = start as usize - songs[i - 2].start as usize;
        }

        songs.push(Song {
            start,
            index: i,
            size: 0,
        });
    }

    // For the last song, use the size of the file as a whole
    let metadata = fs::metadata("Stream.bin")?;
    songs[TRACK_COUNT - 2].size = metadata.len() as usize - songs[TRACK_COUNT - 2].index;

    for song in songs {
        println!(
            "Song {} starts at {} and is {} bytes",
            song.index, song.start, song.size
        );
    }

    Ok(())
}
