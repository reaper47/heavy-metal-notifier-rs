use calendar::Calendar;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

mod calendar;
mod error;
mod scraper;
mod support;
pub mod jobs;
pub mod web;

lazy_static! {
    static ref CALENDAR: Mutex<Calendar> = Mutex::new(Calendar::new());
}