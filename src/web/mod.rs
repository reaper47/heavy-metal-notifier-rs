mod config;
mod handlers_calendar;
mod handlers_general;
mod templates;

use axum::Router;

use crate::error::Result;
use config::web_config;
use handlers_calendar::routes_calendar;
use handlers_general::{routes_general, serve_dir};

pub async fn routes() -> Result<Router> {
    let router = Router::new()
        .merge(routes_general())
        .nest("/calendar", routes_calendar())
        .fallback_service(serve_dir(&web_config().WEB_FOLDER));

    Ok(router)
}
