mod config;
mod handlers_general;
mod templates;

use axum::Router;

use config::web_config;
use crate::error::Result;
use handlers_general::{routes_general, serve_dir};

pub async fn routes() -> Result<Router> {
    let router = Router::new()
        .merge(routes_general())
        .fallback_service(serve_dir(&web_config().WEB_FOLDER));

    Ok(router)
}
