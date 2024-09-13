use std::collections::HashMap;

use reqwest::Url;
use time::OffsetDateTime;
use tracing::info;

use crate::{
    error::Result,
    scraper::{client::Client, wiki::scrape},
};

pub type CalendarData = HashMap<Month, Releases>;

type Day = u8;

pub type Releases = HashMap<Day, Vec<Release>>;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub artist: String,
    pub album: String,
    pub links: Vec<Link>,
}

impl Release {
    pub fn new(artist: impl Into<String>, album: impl Into<String>) -> Self {
        let mut album: String = album.into();
        album = album.split_whitespace().collect::<Vec<&str>>().join(" ");
        if album.contains("[") {
            album = album.split_once('[').unwrap().0.to_string();
        }

        Self {
            artist: artist.into(),
            album,
            links: Vec::new(),
        }
    }

    pub async fn generate_links(&mut self, client: &impl Client) {
        let query = format!("{} {} full album", self.artist, self.album);
        let mut query_encoded = String::new();
        url_escape::encode_query_to_string(query, &mut query_encoded);

        let yt_url = format!("https://www.youtube.com/results?search_query={query_encoded}");
        let yt_url = Url::parse(&yt_url).unwrap();

        self.links.push(Link::Youtube(yt_url));
        /*if let Some(url) = client.get_bandcamp_link(self.artist.clone()).await {
            self.links.push(Link::Bandcamp(url))
        }*/
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Link {
    Bandcamp(Url),
    Youtube(Url),
}

#[derive(Debug, PartialEq)]
pub struct Calendar {
    pub year: i32,
    pub data: CalendarData,
}

impl Calendar {
    pub fn new(year: i32) -> Self {
        Self {
            year,
            data: HashMap::from([
                (Month::January, HashMap::new()),
                (Month::February, HashMap::new()),
                (Month::March, HashMap::new()),
                (Month::April, HashMap::new()),
                (Month::May, HashMap::new()),
                (Month::June, HashMap::new()),
                (Month::July, HashMap::new()),
                (Month::August, HashMap::new()),
                (Month::September, HashMap::new()),
                (Month::October, HashMap::new()),
                (Month::November, HashMap::new()),
                (Month::December, HashMap::new()),
            ]),
        }
    }

    pub fn add_release(&mut self, month: Month, day: Day, release: Release) {
        self.data
            .entry(month)
            .or_insert_with(Releases::new)
            .entry(day)
            .or_insert_with(Vec::new)
            .push(release);
    }

    pub fn get_releases(&self, month: Month, day: Day) -> Option<&Vec<Release>> {
        self.data.get(&month).and_then(|map| map.get(&day))
    }

    pub async fn update(&mut self, client: &impl Client) -> Result<()> {
        info!("Updating calendar");

        let now = OffsetDateTime::now_utc();
        let year = now.year();
        let mut calendar = scrape(client, year).await?;

        let mut releases = Vec::new();
        // Get mutable releases of the month to reduce number of requests to Bandcamp.
        for (_, day_releases) in calendar.data.iter_mut() {
            for (_, release_list) in day_releases.iter_mut() {
                releases.extend(release_list.drain(..))
            }
        }
        for release in &mut releases {
            release.generate_links(client).await;
        }

        self.data = calendar.data;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_default_calendar_empty_ok() -> Result<()> {
        let got = Calendar::new(2024);

        pretty_assertions::assert_eq!(
            got.data,
            CalendarData::from([
                (Month::January, HashMap::new()),
                (Month::February, HashMap::new()),
                (Month::March, HashMap::new()),
                (Month::April, HashMap::new()),
                (Month::May, HashMap::new()),
                (Month::June, HashMap::new()),
                (Month::July, HashMap::new()),
                (Month::August, HashMap::new()),
                (Month::September, HashMap::new()),
                (Month::October, HashMap::new()),
                (Month::November, HashMap::new()),
                (Month::December, HashMap::new()),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_calendar_add_release_ok() -> Result<()> {
        let mut got = Calendar::new(2024);
        let release = Release::new("Wintersun", "Time II");

        got.add_release(Month::August, 30, release.clone());

        let mut want = Calendar::new(2024);
        want.data
            .insert(Month::August, HashMap::from([(30, vec![release])]));
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn test_calendar_get_releases_ok() -> Result<()> {
        let release = Release::new("Wintersun", "Time II");
        let calendar = Calendar {
            year: 2024,
            data: CalendarData::from([(
                Month::August,
                Releases::from([(30, vec![release.clone()])]),
            )]),
        };

        let got = calendar.get_releases(Month::August, 30);

        pretty_assertions::assert_eq!(got, Some(&vec![release]));
        Ok(())
    }
}
