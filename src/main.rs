use std::env;
use url::Url;

/*
  usage - rust-youtube-dl [url]
*/
fn main() {
    let _args: Vec<String> = env::args().collect();
    let url_result = Url::parse(_args[1].as_str());
    let parsed_url = match url_result {
        Ok(url) => url,
        Err(error) => panic!("Oops! Something went wrong: {:?}", error),
    };

    println!("Argument passed {}", parsed_url.host().unwrap());
}