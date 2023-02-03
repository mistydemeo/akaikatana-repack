use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::song::Song;

pub const TRACK_COUNT: usize = 40;
pub const TRACK_HEADER_LENGTH: usize = 8;

pub fn parse_file(file: &mut File) -> Result<Vec<Song>, std::io::Error> {
    let mut buf: Vec<u8> = vec![0; TRACK_COUNT * TRACK_HEADER_LENGTH];
    file.read_exact(&mut buf)?;

    let mut songs: Vec<Song> = vec![];
    for i in 1..=TRACK_COUNT {
        let index = (i - 1) * TRACK_HEADER_LENGTH;

        let mut reader = Cursor::new(&buf);
        reader.set_position(index as u64);

        let start = reader.read_u32::<LittleEndian>().unwrap() as u64;

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
    songs[TRACK_COUNT - 1].size =
        file.metadata()?.len() as usize - songs[TRACK_COUNT - 1].start as usize;

    Ok(songs)
}
