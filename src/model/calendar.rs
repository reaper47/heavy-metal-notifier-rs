use diesel::prelude::*;

use crate::calendar::Calendar;
use crate::error::{Error, Result};

use super::ModelManager;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = super::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = super::schema::links)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Link {
    pub id: i32,
    pub artist_id: i32,
    pub url_youtube: String,
    pub url_bandcamp: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = super::schema::releases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Release {
    pub id: i32,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub artist_id: i32,
    pub album: String,
}

pub struct CalendarBmc;

impl CalendarBmc {
    pub fn create_or_update(calendar: Calendar) -> Result<()> {
        use super::schema::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;
        conn.transaction::<_, Error, _>(|conn| {
            let results = releases::table
                .filter(releases::year.eq(calendar.year))
                .limit(1)
                .select(Release::as_select())
                .load(conn)?;
            
            // Insert artists

            // Insert links 

            // Insert releases

            Ok(())
        })
    }
}