use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

use clap::{App, Arg};
use rocket::http::ContentType;
use rocket::response::NamedFile;
use rocket::{get, post, routes, Rocket};

#[get("/download/<title>")]
fn download(title: String) -> Option<NamedFile> {
    let file_name = format!("{}.mp3", title);
    let file_path = Path::new(&file_name);

    if file_path.exists() {
        return NamedFile::open(file_path).ok();
    }

    let youtube_dl_output = Command::new("youtube-dl")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(&file_name)
        .arg(format!("ytsearch1:{}", title))
        .output()
        .expect("failed to execute youtube-dl");

    if youtube_dl_output.status.success() {
        NamedFile::open(file_path).ok()
    } else {
        None
    }
}

#[post("/upload", data = "<file>")]
fn upload(file: rocket::Data) -> Option<String> {
    let content_type = file.content_type()?;
    if content_type != ContentType::MP3 {
        return None;
    }

    let file_name = format!("{}.mp3", uuid::Uuid::new_v4());
    let file_path = Path::new(&file_name);

    let mut buffer = File::create(file_path).ok()?;
    if let Err(_) = copy(&mut file.open(), &mut buffer) {
        return None;
    }

    Some(file_name)
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![download, upload])
}

fn main() {
    let matches = App::new("Music Downloader")
        .version("1.0")
        .author("Your Name")
        .about("Downloads music from YouTube")
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .value_name("TITLE")
                .help("Sets the title of the music to download")
                .takes_value(true),
        )
        .get_matches();

    if let Some(title) = matches.value_of("title") {
        rocket().launch();
    } else {
        println!("Please provide a title to search for");
    }
