use axum::Router;
use axum::routing::{delete, get, post};
use crate::state::AppState;
use crate::vides::handlers::delete::video_delete_handler;
use crate::vides::handlers::info::video_info_handler;
use crate::vides::handlers::list::video_list_handler;
use crate::vides::handlers::stream::video_stream_handler;
use crate::vides::handlers::upload::video_upload_handler;

pub(crate) fn build_video_routers() ->Router<AppState>{
    Router::new()
        .route("/api/video/list", get(video_list_handler))
        .route("/api/video/list/", get(video_list_handler))
        .route("/api/video/list/{*path}", get(video_list_handler))
        .route("/api/video/stream/{*path}", get(video_stream_handler))
        .route("/api/video/info/{*path}", get(video_info_handler))
        .route("/api/video/delete/{*path}", delete(video_delete_handler))
        .route("/api/video/upload", post(video_upload_handler))
        .route("/api/video/upload/", post(video_upload_handler))
        .route("/api/video/upload/{*path}", post(video_upload_handler))
}