use std::fs::File;

use std::io::Error;

use akaikatana_repack::parser;

fn main() -> Result<(), Error> {
    let mut file = File::open("Stream.bin")?;

    let songs = parser::parse_file(&mut file)?;

    for song in songs {
        println!(
            "Song {} starts at {} and is {} bytes",
            song.index, song.start, song.size
        );
    }

    Ok(())
}
