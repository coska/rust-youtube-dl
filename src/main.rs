use std::env;
use rust_youtube_dl::{parse_url, download_webpage, get_ytplayer_config};
/*
  usage - rust-youtube-dl [url]
*/

fn main() {
    let _args: Vec<String> = env::args().collect();
    let parsed_url = parse_url(_args[1].as_str());
    let contents = download_webpage(parsed_url.as_str());
    get_ytplayer_config(contents);
}