use std::fs::File;

use std::io::Error;

use clap::Parser;

use akaikatana_repack::parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Displays information about the contents of Stream.bin"
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
        println!(
            "Song {} starts at {} and is {} bytes",
            song.index, song.start, song.size
        );
    }

    Ok(())
}
