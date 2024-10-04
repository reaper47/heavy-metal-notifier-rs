use serde::Deserialize;

use crate::{calendar::Calendar, error::Result};

use super::client::Client;

#[derive(Deserialize)]
pub struct MetallumReleases {
    #[serde(rename = "iTotalRecords")]
    pub total_records: i32,
    #[serde(rename = "iTotalDisplayRecords")]
    pub total_display_records: i32,
    #[serde(rename = "aaData")]
    pub data: [String; 6],
}

pub fn scrape(client: &impl Client, year: i32) -> Result<Calendar> {
    let doc = client.get_calendar(year)?;
    Ok(Calendar::new(2025))
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_2024_calendar_ok() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_2025_calendar_ok() -> Result<()> {
        Ok(())
    }
}