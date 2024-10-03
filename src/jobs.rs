//! The `jobs` module implements functions that are meant to be run periodically.

use time::OffsetDateTime;

use crate::{
    error::Result,
    model::CalendarBmc,
    scraper::{client::MainClient, wiki::scrape},
};

/// Fetches, scrapes and updates the heavy metal calendar for the current 
/// year and saves it in the database.
pub async fn update_calendar() -> Result<()> {
    let client = MainClient::new();
    let calendar = scrape(&client, OffsetDateTime::now_utc().year())?;
    CalendarBmc::create_or_update(&client, calendar)?;
    Ok(())
}
