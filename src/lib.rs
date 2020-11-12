use url::Url;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};

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

pub fn download_webpage(url: &str) -> String{
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36"));

    let client: Client = reqwest::blocking::Client::new();
    let res = (match client.get(url)
            .headers(headers)
            .send() {
                Ok(response) => Ok(response),
                Err(error) => Err(error),
            }).unwrap().text().unwrap(); 
    res
}

pub fn get_ytplayer_config(webpage_contents: String) {
    let patterns = [
        r";ytplayer\.config\s*=\s*({.+?});ytplayer",
        r";ytplayer\.config\s*=\s*({.+?});",
    ];

    let mut ytplayer_config = "";

    for pattern in patterns.iter() {
        let re = Regex::new(pattern).unwrap();
        for captured in re.captures_iter(webpage_contents.as_str()) {
            println!("{}", &captured[0]);
            break;
            //ytplayer_config = &captured[0];
        }
    }

}


