use diesel::prelude::*;

use crate::error::Result;

use super::{schema, ModelManager};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = super::schema::feeds)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feed {
    pub id: i32,
    pub date: i32,
    pub feed: String,
}

pub struct FeedForCreate {
    pub date: i32,
    pub feed: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::feeds)]
struct FeedForInsert {
    pub date: i32,
    pub feed: String,
}

pub struct FeedBmc;

impl FeedBmc {
    pub fn create(feed_c: FeedForCreate) -> Result<()> {
        use schema::feeds::dsl::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        diesel::insert_or_ignore_into(feeds)
            .values(&FeedForInsert {
                date: feed_c.date,
                feed: feed_c.feed,
            })
            .execute(conn)?;

        Ok(())
    }

    pub fn get(num: i64) -> Result<Vec<Feed>> {
        use schema::feeds::dsl::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        let results = feeds
            .order(date.desc())
            .limit(num)
            .select(Feed::as_select())
            .load(conn)?;

        Ok(results)
    }
}
