use diesel::prelude::*;
use time::OffsetDateTime;

use crate::calendar::Calendar;
use crate::error::{Error, Result};

use super::ModelManager;

/// This struct corresponds to a row in the `artists` 
/// table in the database. Each artist has a unique `id` and 
/// a `name`.
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = super::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub name: String,
}


/// Represents web links associated with an artist.
/// 
/// This struct corresponds to a row in the `links` table, 
/// which stores external links related to the artist 
/// (such as YouTube and Bandcamp).
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

/// Represents a music release by an artist.
///
/// This struct corresponds to a row in the `releases` table.
/// It stores information about an artist's album release, 
/// including the release date (year, month, day) and the album's 
/// title.
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

/// Represents a new release to be inserted into the database.
///
/// This struct is used when creating new records in the `releases` table. 
/// It doesn't include the `id` field because the database will generate it.
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

/// `CalendarBmc` is a backend model controller responsible for 
/// managing calendar-related operations.
///
/// It provides methods to create, update, and retrieve calendar 
/// data, including releases and associated links.
pub struct CalendarBmc;

impl CalendarBmc {
    /// Creates or updates a calendar with the provided data.
    ///
    /// This method inserts new releases into the `releases` table
    /// or updates existing ones based on the calendar data. It 
    /// handles linking artists and adding external links (YouTube, Bandcamp).
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
                        let artist_name = release.artist.clone();

                        let artist_id: i32 = match diesel::insert_or_ignore_into(artists::table)
                            .values(artists::name.eq(&artist_name))
                            .returning(artists::id)
                            .get_result(conn)
                        {
                            Ok(id) => id,
                            Err(_) => artists::table
                                .filter(artists::name.eq(&artist_name))
                                .limit(1)
                                .select(artists::id)
                                .get_result(conn)?,
                        };

                        if CalendarBmc::get_links(conn, &artist_name).is_none() {
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
                        }

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

    /// Retrieves links associated with an artist.
    ///
    /// This method queries the `links` table to fetch YouTube 
    /// and Bandcamp URLs associated with a given artist.
    pub fn get_links(conn: &mut SqliteConnection, artist: impl Into<String>) -> Option<Vec<Link>> {
        use super::schema::*;

        let links: core::result::Result<Vec<Link>, _> = links::table
            .inner_join(artists::table)
            .filter(artists::name.eq(artist.into()))
            .select(Link::as_select())
            .load(conn);

        match links {
            Ok(vec) => {
                if vec.is_empty() {
                    None
                } else {
                    Some(vec)
                }
            }
            Err(_) => None,
        }
    }

    /// Retrieves releases for the current date.
    ///
    /// This method fetches releases from the `releases` table 
    /// that match the current date (year, month, and day) and 
    /// joins the associated artist and links (YouTube, Bandcamp).
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
