use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::IntoResponse,
    routing::{any_service, get, MethodRouter},
    Router,
};
use maud::Markup;
use reqwest::header::CONTENT_TYPE;
use rss::{ChannelBuilder, Guid, ItemBuilder};
use time::OffsetDateTime;
use tower_http::services::ServeDir;
use tracing::error;

use crate::{model::CalendarBmc, web::templates};

use super::{config::web_config, templates::main::*};

pub fn routes_general() -> Router {
    Router::new()
        .route("/", get(index()))
        .route("/about", get(about()))
        .route("/contact", get(contact(false)).post(contact(true)))
        .route("/feed", get(feed))
        .route("/privacy", get(privacy()))
        .route("/tos", get(tos()))
        .nest_service("/static", ServeDir::new(&web_config().WEB_FOLDER))
}

pub fn serve_dir(web_folder: &'static String) -> MethodRouter {
    async fn handle_404() -> (StatusCode, Markup) {
        (
            StatusCode::NOT_FOUND,
            templates::general::simple(
                "Page Not Found",
                "The page you requested to view is not found. Please go back to the main page.",
            ),
        )
    }

    any_service(ServeDir::new(web_folder).not_found_service(handle_404.into_service()))
}

async fn feed() -> impl IntoResponse {
    match CalendarBmc::get() {
        Ok(releases) => {
            let now = OffsetDateTime::now_utc();
            let now = now+time::Duration::days(1);
            let pub_date = match now.format(&time::format_description::well_known::Rfc2822) {
                Ok(date) => date,
                Err(_) => String::new(),
            };

            let content = releases.iter().fold(
                "".to_string(),
                |mut acc, (release, artist, (url_youtube, url_bandcamp))| {
                    acc.push_str(&format!("{} - {}<br/>", artist.name, release.album));
                    
                    acc.push_str(&format!("&emsp;• <a href=\"{}\">Youtube</a><br/>", url_youtube));
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
                    .link("/feed")
                    .build()
            } else {
                let date = format!("{} {}, {}", now.month(), now.day(), now.year());
                let mut guid = Guid::default();
                guid.set_value(&format!("{}", date));


                let item = ItemBuilder::default()
                    .title(date.clone())
                    .pub_date(pub_date.clone())
                    .content(content)
                    .guid(guid)
                    .build();

                ChannelBuilder::default()
                    .title("Heavy Metal Releases")
                    .description("A feed for the latest heavy metal album releases.")
                    .pub_date(pub_date)
                    .link("/feed")
                    .item(item)
                    .build()
            };

            ([(CONTENT_TYPE, "application/rss+xml")], channel.to_string()).into_response()
        }
        Err(err) => {
            error!(
                "getting releases today {}: {}",
                OffsetDateTime::now_utc(),
                err
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch today's releases.",
            )
                .into_response()
        }
    }
}
