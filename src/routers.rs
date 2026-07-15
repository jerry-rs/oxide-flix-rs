use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::http::{header, Response, StatusCode, Uri};
use axum::response::{IntoResponse};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get};
use rust_embed::Embed;
use tracing::info;
use crate::vides::routers::build_video_routers;

// #[derive(Template)]
// #[template(path = "index.html")]
// struct IndexTemplate;
// async fn index_handler() -> impl IntoResponse {
//     match IndexTemplate.render() {
//         Ok(html) => Html(html).into_response(),
//         Err(e) => {
//             error!("{e}");
//             (StatusCode::INTERNAL_SERVER_ERROR).into_response()
//         },
//     }
// }


#[derive(Embed)]
#[folder = "dist/"]
struct Asset;

async fn index_handler(
    uri: Uri
)->impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    // 处理空路径，默认指向 index.html
    let asset_path = if path.is_empty() {
        "index.html"
    } else {
        path
    };
    info!("{asset_path}");
    match Asset::get(asset_path) {
        Some(content) => {
            let mime = content.metadata.mimetype();
            Response::builder()
                .header(header::CONTENT_TYPE, mime)
                .body(Body::from(content.data))
                .unwrap()
        },
        None => {
            if let Some(index) = Asset::get("index.html") {
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(Body::from(index.data))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap()
            }
        },
    }
}

pub(crate) fn build_global_routers(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/{*path}", get(index_handler))
        .merge(build_video_routers())
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(DefaultBodyLimit::disable())
        .with_state(state)
}
