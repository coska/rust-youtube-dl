use std::cell::RefCell;
use url::Url;
use json::JsonValue;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

pub struct StreamingData {
    itag: i32,
    content_length: String,
    url: String,
}

pub struct VideoInfo {
    title: String,
    description: String,
    author: String,
    streaming_data: RefCell<Vec<StreamingData>>
}

impl VideoInfo {
    fn append_streaming_data(&self, value: StreamingData) {
        self.streaming_data.borrow_mut().push(value)
    }
}

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

pub fn get_ytplayer_config(webpage_contents: String) -> Result<String, &'static str> {
    let patterns = [
        r"var ytInitialPlayerResponse\s*=\s*(\{.+?\});"
        //r";ytplayer\.config\s*=\s*(\{.+?\});ytplayer",
        //r";ytplayer\.config\s*=\s*(\{.+?\});",
    ];

    let mut ytplayer_config = "";

    for pattern in patterns.iter() {
        let re = Regex::new(pattern).unwrap();
        for captured in re.captures_iter(webpage_contents.as_str()) {
            ytplayer_config = captured.get(1).unwrap().as_str();
            break;
        }
    }
    if ytplayer_config.is_empty() {
        return Err("No ytplayer config found");
    }
    Ok(ytplayer_config.to_string())
}

pub fn extract_streaming_data(player_config: JsonValue) {
    /* player_config = {
        'videoDetails': {
            'title': String,
            'shortDescription': String,
            'author': String
        },
        streamingData: {
            'format': {
                'url': 'https://r2---sn-cx5o4aqj5-tt1e.googlevideo.com/videoplayback?expire=1606984012&ei=7EzIX5uQCIX7igTEy52oCQ&ip=72.53.212.162&id=o-AI7mo8fvoYlhwGGc8uIAUYU_IlHART5tdDaZZtX8eBSL&itag=18&source=youtube&requiressl=yes&mh=ts&mm=31%2C29&mn=sn-cx5o4aqj5-tt1e%2Csn-tt1eln7l&ms=au%2Crdu&mv=m&mvi=2&pl=24&initcwndbps=1315000&vprv=1&mime=video%2Fmp4&ns=Z9UIxsr06sGfd0PV4rpuXSAF&gir=yes&clen=24254355&ratebypass=yes&dur=387.517&lmt=1604438816797344&mt=1606962033&fvip=2&c=WEB&txp=5432434&n=WwYO9TTay3OAXq6gw&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cns%2Cgir%2Cclen%2Cratebypass%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAN012scsafErlrOnMYZGVky4PtjAYCIle9JQIgXD-JidAiBxSbsArdgG6vx14-YQ1Ty-A8C4r4NKlGzZq3kwR_Lz4g%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRgIhAK80NHx7muczkr3y5I8n9I1IuSbI23w2QY7FDygp6oEKAiEA7XI2IwfURThuzzsKc7ctrLlA3JBmidUAhgDgVI_B83k%3D',
                'itag': 18,
                'mimetype": 'video/mp4',
                'contentLength': '1223123'
            }
        }
    }*/
    //let video_details = player_config["videoDetails"];
    /*
    let mut video_info = VideoInfo {
        title: video_details["title"],
        description: video_details["shortDescription"],
        author: video_details["author"],
        streaming_data: RefCell::new(Vec::new())
    }
    */
}