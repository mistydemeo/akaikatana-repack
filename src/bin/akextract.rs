use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use std::io::Error;

use akaikatana_repack::parser;

fn main() -> Result<(), Error> {
    let mut file = File::open("Stream.bin")?;

    let songs = parser::parse_file(&mut file)?;

    for song in songs {
        let filename = format!("{:0>2}.wav", song.index);
        let mut buf: Vec<u8> = vec![0; song.size];

        file.seek(SeekFrom::Start(song.start))?;
        file.read_exact(&mut buf)?;

        println!("Writing song {filename}");
        let mut out = File::create(filename)?;
        out.write_all(&buf)?;
    }

    Ok(())
}
