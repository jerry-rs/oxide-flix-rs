use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get};
use crate::vides::routers::build_video_routers;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;
async fn index_handler() -> impl IntoResponse {
    match IndexTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub(crate) fn build_global_routers(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .merge(build_video_routers())
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state)
}
