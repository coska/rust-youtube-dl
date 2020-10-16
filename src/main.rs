use std::env;
use rust_youtube_dl::parse_url;
/*
  usage - rust-youtube-dl [url]
*/

fn main() {
    let _args: Vec<String> = env::args().collect();
    let parsed_url = parse_url(_args[1].as_str());
}