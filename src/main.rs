use std::env;
use rust_youtube_dl::{parse_url, download_webpage, get_ytplayer_config};
use json;
/*
  usage - rust-youtube-dl [url]
*/

fn main() {
    let _args: Vec<String> = env::args().collect();
    let parsed_url = parse_url(_args[1].as_str());
    let contents = download_webpage(parsed_url.as_str());
    let player_config = get_ytplayer_config(contents);
    let parsed_config = json::parse(&player_config).unwrap();
    println!("Renderer {}", parsed_config["captions"]["playerCaptionsTracklistRenderer"])
}