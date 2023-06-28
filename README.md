## Akai Katana music repacker

This set of tools allows you to extract and replace the music from the [Steam release of Akai Katana](https://store.steampowered.com/app/2076220/Akai_Katana_Shin/). I was inspired to make it because I love the FM arranged soundtrack from the Nin2 Jump/Akai Katana CD, but there's no way to actually use it in game. If there won't be an official one, why not make my own?

### Getting the tool

Prebuilt copies of the tool can be found in the ["releases" tab](https://github.com/mistydemeo/akaikatana-repack/releases) in GitHub. You should be able to just download and run the ZIP file or tarball.

### How to use

Akai Katana's music is in a file called `Stream.bin` within the game folder. To start, copy it into the folder with these tools.

The `akextract` tool is used to extract the music from `Stream.bin`. After extracting, you'll have a set of 40 `.wav` files with all the music from all three versions of the soundtrack. The `tracklist.txt` file in the documents folder explains which file is which song.

To replace songs with new ones, you'll need to reencode the new file in Microsoft's ADPCM format. Doing this requires Microsoft's commandline tool `AdpcmEncode.exe`. The game won't accept normal WAV files, and it won't accept ADPCM files made with any other tool (like FFmpeg).

To let the music loop properly, you can use a tool such as [Wavosaur](https://www.wavosaur.com); instructions for editing loop points are [here](https://www.wavosaur.com/quick-help/loop-points-edition.php). `AdpcmEncode.exe` will use the loop points embedded into the uncompressed WAV files. If you're using the FM arrange from the Nin2 Jump/Akai Katana CD, I've included a set of loop points in the `loop_points.txt` file in the documents folder.

Once you're done preparing your music, place your new ADPCM WAV files in a folder with any of the original music you're keeping. Your new folder should still have 40 WAV files with the same numbered names as the original ones. Once you're ready, you can run the `akrepack` tool to repackage your files into a new `Stream.bin`. For example, if your new soundtrack is in a folder called `in`, you would run `akrepack --input in`. This will produce a new file called `Stream.bin.repacked`. Just replace the file in Akai Katana's game folder with this new file, and you're ready to go.

### Building from source

These tools are written in Rust, and can be built using standard Rust tools. As long as you have the Rust compiler installed, you just need to run `cargo build`.

### Contributing

Contributions are always welcome! Please open issues if you run into any issues, and pull requests are always a big help.

### License

GPL 2.0 or later.
