use std::collections::HashMap;

use reqwest::Url;

use crate::scraper::client::Client;

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
}

#[derive(Clone, Debug, PartialEq)]
pub enum Link {
    Bandcamp(Url),
    Youtube(Url),
}

#[derive(Debug, PartialEq)]
pub struct Calendar {
    pub data: CalendarData,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_default_calendar_empty_ok() -> Result<()> {
        let got = Calendar::new();

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
        let mut got = Calendar::new();
        let release = Release::new("Wintersun", "Time II");

        got.add_release(Month::August, 30, release.clone());

        let mut want = Calendar::new();
        want.data
            .insert(Month::August, HashMap::from([(30, vec![release])]));
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn test_calendar_get_releases_ok() -> Result<()> {
        let release = Release::new("Wintersun", "Time II");
        let calendar = Calendar {
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
