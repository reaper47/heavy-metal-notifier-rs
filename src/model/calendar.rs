use diesel::prelude::*;
use time::OffsetDateTime;

use crate::calendar::Calendar;
use crate::config::config;
use crate::error::{Error, Result};
use crate::scraper::client::Client;

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
    pub url_bandcamp: Option<String>,
    pub url_metallum: String,
}

/// Represents a new artist to be inserted into the database.
///
/// This struct is used when creating new records in the `artists` table.
/// It doesn't include the `id` field because the database will generate it.
#[derive(Insertable)]
#[diesel(table_name = super::schema::artists)]
struct ArtistForInsert {
    pub name: String,
    pub url_bandcamp: Option<String>,
    pub url_metallum: String,
}

impl ArtistForInsert {
    pub fn new(client: &impl Client, name: impl Into<String>) -> Self {
        let name: String = name.into();
        let url_name = name.replace(" ", "_");

        let url_bandcamp = if config().IS_PROD {
            if let Some(url) = client.get_bandcamp_link(name.clone()) {
                Some(url.to_string())
            } else {
                None
            }
        } else {
            None
        };

        Self {
            name: name.clone(),
            url_bandcamp,
            url_metallum: format!("https://www.metal-archives.com/bands/{url_name}"),
        }
    }
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
    pub url_youtube: String,
    pub url_metallum: String,
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
    pub url_youtube: String,
    pub url_metallum: String,
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
    pub fn create_or_update(client: &impl Client, calendar: Calendar) -> Result<()> {
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
                            .values(&ArtistForInsert::new(client, &artist_name))
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

                        let query = format!("{} {} full album", artist_name, release.album.clone());
                        let mut query_encoded = String::new();
                        url_escape::encode_query_to_string(query, &mut query_encoded);
                        let url_youtube =
                            format!("https://www.youtube.com/results?search_query={query_encoded}");

                        let name_metallum = artist_name.replace(" ", "_");
                        let album_metallum = release.album.clone().replace(" ", "_");
                        let url_metallum = format!(
                            "https://www.metal-archives.com/bands/{name_metallum}/{album_metallum}"
                        );

                        diesel::insert_into(releases::table)
                            .values(&ReleaseForInsert {
                                year: calendar.year,
                                month: *month as i32,
                                day: *day as i32,
                                artist_id,
                                album: release.album.clone(),
                                url_youtube,
                                url_metallum,
                            })
                            .execute(conn)?;
                    }
                }
            }

            Ok(())
        })
    }

    /// Retrieves releases for the current date.
    ///
    /// This method fetches releases from the `releases` table
    /// that match the current date (year, month, and day) and
    /// joins the associated artist and links (YouTube, Bandcamp).
    pub fn get() -> Result<Vec<(Release, Artist)>> {
        use super::schema::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        let now = OffsetDateTime::now_utc();
        let year = now.year();
        let month = now.month() as i32;
        let day = now.day() as i32;

        let releases = releases::table
            .inner_join(artists::table)
            .filter(
                releases::year
                    .eq(year)
                    .and(releases::month.eq(month))
                    .and(releases::day.eq(day)),
            )
            .select((Release::as_select(), Artist::as_select()))
            .load(conn)?;

        Ok(releases)
    }
}
