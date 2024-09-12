use axum::async_trait;
use scraper::Html;

use crate::error::Result;

pub struct MainClient;

impl MainClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
pub trait Client {
    async fn get(&self, year: i32) -> Result<scraper::Html>;
}

#[async_trait]
impl Client for MainClient {
    async fn get(&self, year: i32) -> Result<scraper::Html> {
        let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
        let res = reqwest::get(url).await?;
        let text = res.text().await?;
        Ok(Html::parse_document(text.as_str()))
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
        pub async fn scrape(&self, year: i32) -> Result<Calendar> {
            scrape(self, year).await
        }
    }

    #[cfg(test)]
    #[async_trait]
    impl Client for MockClient {
        async fn get(&self, year: i32) -> Result<scraper::Html> {
            let path = PathBuf::from(format!("./tests/testdata/test_{year}.html"));
    
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => {
                    let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
                    match reqwest::get(url).await {
                        Ok(res) => {
                            let mut file = fs::File::create(path)?;
                            let content = res.text().await?;
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
    }    
}

