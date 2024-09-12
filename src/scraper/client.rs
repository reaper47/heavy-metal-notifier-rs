use std::{fs, io::Write, path::PathBuf};

use axum::async_trait;
use scraper::Html;

use crate::{
    calendar::Calendar,
    error::{Error, Result},
    scraper::wiki::extract_calendar,
};

pub struct MockClient;

impl MockClient {
    pub async fn scrape(&self, year: u16) -> Result<Calendar> {
        scrape(self, year).await
    }
}

async fn scrape(client: &impl Client, year: u16) -> Result<Calendar> {
    let doc = client.get(year).await?;
    Ok(extract_calendar(doc))
}

#[async_trait]
pub trait Client {
    async fn get(&self, year: u16) -> Result<scraper::Html>;
}

#[async_trait]
impl Client for MockClient {
    async fn get(&self, year: u16) -> Result<scraper::Html> {
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
