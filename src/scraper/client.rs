use reqwest::Url;
use scraper::Html;

use crate::error::Result;

pub struct MainClient;

impl MainClient {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Client {
    fn get_calendar(&self, year: i32) -> Result<scraper::Html>;
    fn get_bandcamp_link(&self, artist: String) -> Option<Url>;
}

impl Client for MainClient {
    fn get_calendar(&self, year: i32) -> Result<scraper::Html> {
        let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
        let res = reqwest::blocking::get(url)?;
        let text = res.text()?;
        Ok(Html::parse_document(text.as_str()))
    }

    fn get_bandcamp_link(&self, artist: String) -> Option<Url> {
        let artist = artist
            .to_lowercase()
            .replace(":", "")
            .split_whitespace()
            .collect::<String>();
        let url = format!("https://{artist}.bandcamp.com");
        let res = match reqwest::blocking::get(&url) {
            Ok(res) => {
                let req_url = res.url().path();
                let req_host = res.url().host_str().unwrap_or("");
                if req_url != "/signup" || req_host == format!("{}.bandcamp.com", artist) {
                    Some(Url::parse(&url).unwrap())
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        std::thread::sleep(std::time::Duration::from_millis(200));
        res
    }
}

#[cfg(test)]
pub mod tests {
    use std::{fs, io::Write, path::PathBuf};

    use crate::{calendar::Calendar, error::Error, scraper::wiki::scrape};

    use super::*;

    #[cfg(test)]
    pub struct MockClient;

    #[cfg(test)]
    impl MockClient {
        pub fn new() -> Self {
            Self {}
        }

        pub fn scrape(&self, year: i32) -> Result<Calendar> {
            scrape(self, year)
        }
    }

    impl Client for MockClient {
        fn get_calendar(&self, year: i32) -> Result<scraper::Html> {
            let path = PathBuf::from(format!("./tests/testdata/test_{year}.html"));

            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => {
                    let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
                    match reqwest::blocking::get(url) {
                        Ok(res) => {
                            let mut file = fs::File::create(path)?;
                            let content = res.text()?;
                            if let Err(err) = file.write(&content.as_bytes()) {
                                return Err(Error::Io(err));
                            }
                            content
                        }
                        Err(_) => return Err(Error::RequestFail),
                    }
                }
            };

            Ok(Html::parse_document(&content))
        }

        fn get_bandcamp_link(&self, artist: String) -> Option<Url> {
            let artist = artist
                .to_lowercase()
                .replace(":", "")
                .split_whitespace()
                .collect::<String>();
            let url = format!("https://{artist}.bandcamp.com");
            println!("{url}");
            Some(Url::parse(&url).unwrap())
        }
    }
}
