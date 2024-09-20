use diesel::prelude::*;
use time::OffsetDateTime;

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

#[derive(Insertable)]
#[diesel(table_name = super::schema::links)]
struct LinkForInsert {
    artist_id: i32,
    url_youtube: String,
    url_bandcamp: Option<String>,
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

#[derive(Insertable, Associations)]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = super::schema::releases)]
struct ReleaseForInsert {
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
            diesel::delete(releases::table.filter(releases::year.eq(calendar.year)))
                .execute(conn)?;

            for (month, data) in calendar.data.iter() {
                for (day, releases) in data.iter() {
                    for release in releases.iter() {
                        let artist_id: i32 = diesel::insert_or_ignore_into(artists::table)
                            .values(artists::name.eq(release.artist.clone()))
                            .returning(artists::id)
                            .get_result(conn)?;

                        let mut link_for_insert = LinkForInsert {
                            artist_id,
                            url_youtube: String::new(),
                            url_bandcamp: None,
                        };

                        for link in release.links.iter() {
                            match link {
                                crate::calendar::Link::Bandcamp(url) => {
                                    link_for_insert.url_bandcamp = Some(url.to_string())
                                }
                                crate::calendar::Link::Youtube(url) => {
                                    link_for_insert.url_youtube = url.to_string()
                                }
                            };
                        }

                        diesel::insert_or_ignore_into(links::table)
                            .values(&link_for_insert)
                            .execute(conn)?;

                        diesel::insert_into(releases::table)
                            .values(&ReleaseForInsert {
                                year: calendar.year,
                                month: *month as i32,
                                day: *day as i32,
                                artist_id,
                                album: release.album.clone(),
                            })
                            .execute(conn)?;
                    }
                }
            }

            Ok(())
        })
    }

    pub fn get() -> Result<Vec<(Release, Artist, (String, Option<String>))>> {
        use super::schema::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        let now = OffsetDateTime::now_utc();
        let year = now.year();
        let month = now.month() as i32;
        let day = now.day() as i32;

        let releases = releases::table
            .inner_join(artists::table.inner_join(links::table))
            .filter(
                releases::year
                    .eq(year)
                    .and(releases::month.eq(month))
                    .and(releases::day.eq(day)),
            )
            .select((
                Release::as_select(),
                Artist::as_select(),
                (links::url_youtube, links::url_bandcamp),
            ))
            .load(conn)?;

        Ok(releases)
    }
}
