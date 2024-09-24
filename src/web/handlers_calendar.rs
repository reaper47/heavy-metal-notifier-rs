use axum::{response::IntoResponse, routing::get, Router};
use reqwest::{header::CONTENT_TYPE, StatusCode};
use rss::{Channel, ChannelBuilder, Guid, Item, ItemBuilder};
use time::OffsetDateTime;
use tracing::error;

use crate::config::config;
use crate::error::Result;
use crate::model::{CalendarBmc, FeedBmc, FeedForCreate};

pub fn routes_calendar() -> Router {
    Router::new().route("/feed.xml", get(feed))
}

async fn feed() -> impl IntoResponse {
    let now = OffsetDateTime::now_utc();
    let date_int = match format!("{}{}{}", now.year(), now.month() as u8, now.day(),).parse::<i32>()
    {
        Ok(n) => n,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse today's date.",
            )
                .into_response()
        }
    };
    let pub_date = match now.format(&time::format_description::well_known::Rfc2822) {
        Ok(date) => date,
        Err(_) => String::new(),
    };
    let date = format!("{} {}, {}", now.month(), now.day(), now.year());

    match FeedBmc::get(12) {
        Ok(feeds) => {
            let mut items = feeds
                .iter()
                .map(|f| {
                    match Channel::read_from(f.feed.as_bytes()) {
                        Ok(channel) => {
                            channel.items.get(0).unwrap().clone() // Unwrap used here because a successful channel read always contains an item
                        }
                        Err(err) => {
                            error!("Error reading channel item: {err}");
                            Item::default()
                        }
                    }
                })
                .collect::<Vec<Item>>();

            let image = rss::ImageBuilder::default()
                .link(format!("{}/static/favicon.png", config().BASE_URL))
                .build();

            let channel = match feeds.get(0) {
                Some(feed) => {
                    if feed.date == date_int {
                        ChannelBuilder::default()
                            .title("Heavy Metal Releases")
                            .description("A feed for the latest heavy metal album releases.")
                            .pub_date(pub_date)
                            .link("/calendar/feed.xml")
                            .image(image)
                            .items(items)
                            .build()
                    } else {
                        match create_new_feed(pub_date.clone(), date, date_int) {
                            Ok(channel) => {
                                if let Some(item) = channel.items.get(0) {
                                    items.insert(0, item.clone());
                                }

                                ChannelBuilder::default()
                                    .title("Heavy Metal Releases")
                                    .description(
                                        "A feed for the latest heavy metal album releases.",
                                    )
                                    .pub_date(pub_date)
                                    .link("/calendar/feed.xml")
                                    .image(image)
                                    .items(items)
                                    .build()
                            }
                            Err(err) => {
                                error!("Error creating new channel: {err}");
                                build_default_channel(pub_date)
                            }
                        }
                    }
                }
                None => match create_new_feed(pub_date.clone(), date, date_int) {
                    Ok(channel) => channel,
                    Err(err) => {
                        error!("Error creating new channel: {err}");
                        build_default_channel(pub_date)
                    }
                },
            };

            (
                [(CONTENT_TYPE, "text/xml;charset=UTF-8")],
                channel.to_string(),
            )
                .into_response()
        }
        Err(err) => {
            error!(
                "getting releases today {}: {err}",
                OffsetDateTime::now_utc()
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch today's releases.",
            )
                .into_response();
        }
    }
}

fn create_new_feed(pub_date: String, date: String, date_int: i32) -> Result<Channel> {
    match CalendarBmc::get() {
        Ok(releases) => {
            let content = releases.iter().fold(
                "".to_string(),
                |mut acc, (release, artist, (url_youtube, url_bandcamp))| {
                    acc.push_str(&format!("{} - {}<br/>", artist.name, release.album));

                    acc.push_str(&format!(
                        "&emsp;• <a href=\"{}\">Youtube</a><br/>",
                        url_youtube
                    ));
                    if let Some(url) = url_bandcamp {
                        acc.push_str(&format!("&emsp;• <a href=\"{}\">Bandcamp</a><br/>", url));
                    }
                    acc.push_str("<br/>");

                    acc
                },
            );

            let channel = if content.is_empty() {
                ChannelBuilder::default()
                    .title("Heavy Metal Releases")
                    .description("A feed for the latest heavy metal album releases.")
                    .pub_date(pub_date.clone())
                    .last_build_date(pub_date)
                    .language("en-US".to_string())
                    .link("/calendar/feed.xml")
                    .build()
            } else {
                let mut guid = Guid::default();
                guid.set_value(&format!("{}", date));

                let item = ItemBuilder::default()
                    .title(date.clone())
                    .pub_date(pub_date.clone())
                    .content(content)
                    .guid(guid)
                    .build();

                let channel = ChannelBuilder::default()
                    .title("Heavy Metal Releases")
                    .description("A feed for the latest heavy metal album releases.")
                    .pub_date(pub_date)
                    .link("/calendar/feed.xml")
                    .item(item)
                    .build();

                let xml = channel.to_string();
                if let Err(err) = FeedBmc::create(FeedForCreate {
                    date: date_int,
                    feed: xml.clone(),
                }) {
                    error!("Error creating feed: feed={xml}, error={err}")
                }

                channel
            };

            Ok(channel)
        }
        Err(err) => {
            error!("Error fetching calendar: {}", err);
            Err(err)
        }
    }
}

fn build_default_channel(pub_date: String) -> Channel {
    ChannelBuilder::default()
        .title("Heavy Metal Releases")
        .description("A feed for the latest heavy metal album releases.")
        .pub_date(pub_date.clone())
        .last_build_date(pub_date)
        .language("en-US".to_string())
        .link("/calendar/feed.xml")
        .build()
}
