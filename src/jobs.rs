use time::OffsetDateTime;

use crate::{error::Result, model::CalendarBmc, scraper::{client::MainClient, wiki::scrape}};

pub async fn update_calendar() -> Result<()> {
    let client = MainClient::new();
    let calendar = scrape(&client, OffsetDateTime::now_utc().year()).await?;
    CalendarBmc::create_or_update(calendar)?;
    Ok(())
}
