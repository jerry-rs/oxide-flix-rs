use axum::Router;
use axum::routing::{delete, get};
use crate::state::AppState;
use crate::vides::handlers::delete::video_delete_handler;
use crate::vides::handlers::info::video_info_handler;
use crate::vides::handlers::list::video_list_handler;
use super::handlers::stream::video_stream_handler;

pub(crate) fn build_video_routers() ->Router<AppState>{
    Router::new()
        .route("/api/video/list", get(video_list_handler))
        .route("/api/video/list/", get(video_list_handler))
        .route("/api/video/list/{*path}", get(video_list_handler))
        .route("/api/video/stream/{*path}", get(video_stream_handler))
        .route("/api/video/info/{*path}", get(video_info_handler))
        .route("/api/video/delete/{*path}", delete(video_delete_handler))
}