# music-rust
Working with CHATGPT to create Music downloader

This modified code uses the clap crate to parse command-line arguments. It expects a title argument to be provided, which is used to search for the music on YouTube.

The download endpoint now takes the title as a parameter instead of the URL. It uses the ytsearch1 option of youtube-dl to search for the music on YouTube and download the first result.

To use this code, you can run the following command:

```Bash
cargo run -- --title "Despacito"

```

This will download the MP3 file for the song "Despacito" and save it to the current directory.
