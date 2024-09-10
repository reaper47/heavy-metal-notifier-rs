use axum::{
    handler::HandlerWithoutStateExt, http::StatusCode, routing::{any_service, get, MethodRouter}, Router
};
use maud::Markup;
use tower_http::services::ServeDir;

use crate::web::templates;

use super::{config::web_config, templates::main::*};

pub fn routes_general() -> Router {
    Router::new()
        .route("/", get(index()))
        .route("/about", get(about()))
        .route("/contact", get(contact(false)).post(contact(true)))
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
