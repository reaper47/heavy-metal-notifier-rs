mod calendar;
mod feed;
mod store;

pub(in crate::model) mod schema;
pub use calendar::CalendarBmc;
pub use feed::{Feed, FeedBmc, FeedForCreate};

use diesel::prelude::*;

use crate::config::config;
use store::establish_connection;

pub struct ModelManager {
    pub conn: SqliteConnection,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            conn: establish_connection(&config().DATABASE_URL),
        }
    }
}
