use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

use std::io::Error;

use akaikatana_repack::parser;
use akaikatana_repack::song::Song;

fn main() -> Result<(), Error> {
    let mut out = File::create("Stream.bin.repacked")?;
    // 320 bytes for the original file
    let mut start = parser::TRACK_COUNT * parser::TRACK_HEADER_LENGTH;

    // Start by writing an empty header; we'll go back and fill that
    // in later with data from the files.
    let buf = vec![0; start];
    out.write_all(&buf)?;

    let song_root = Path::new("in");

    let mut songs = vec![];
    for i in 1..=parser::TRACK_COUNT {
        let filename = format!("{:0>2}.wav", i);
        let path = song_root.join(filename);
        let mut file = File::open(path)?;
        let real_size = file.metadata()?.len() as usize;
        let size = ((real_size + 15) / 16) * 16;
        println!("Real size: {}; padded size: {}", real_size, size);

        // Read from the source, then write as-is to the target
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;

        // Pad out to a multiple of 16
        buf.append(&mut vec![0; size - real_size]);

        out.write_all(&buf)?;

        songs.push(Song {
            index: i,
            size: size,
            start: start as u64,
            real_size: real_size,
        });

        // Increment for the next song
        start = start + size;
    }

    // Rewind to the beginning so we can write the header
    out.seek(SeekFrom::Start(0))?;
    for song in songs {
        out.write_all(&song.as_header())?;
    }

    println!("Done!");

    Ok(())
}
