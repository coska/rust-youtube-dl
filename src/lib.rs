use url::Url;
use regex::Regex;

pub fn parse_url(url_str: &str) -> Url {
    let re = Regex::new(r"^.*(?:(?:youtu(?:\.be|be\.com)/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
    
    if !re.is_match(url_str) {
        panic!("Not an youtube url");
    }

    let parsed_url = match Url::parse(url_str) {
        Ok(url) => url,
        Err(error) => panic!("Oops! Something went wrong: {:?}", error),
    };

    parsed_url
}