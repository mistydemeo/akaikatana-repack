use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use std::io::Error;

use clap::Parser;

use akaikatana_repack::parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Extract audio files from Stream.bin"
)]
struct Args {
    #[arg(default_value = "Stream.bin")]
    filename: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut file = File::open(args.filename)?;

    let songs = parser::parse_file(&mut file)?;

    for song in songs {
        let filename = format!("{:0>2}.wav", song.index);
        let mut buf: Vec<u8> = vec![0; song.real_size];

        file.seek(SeekFrom::Start(song.start))?;
        file.read_exact(&mut buf)?;

        println!("Writing song {filename}");
        let mut out = File::create(filename)?;
        out.write_all(&buf)?;
    }

    Ok(())
}
