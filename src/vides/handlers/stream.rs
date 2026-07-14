use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tower::ServiceExt;
use tracing::error;

pub(crate) async fn video_stream_handler(
    State(state): State<AppState>,
    axum::extract::Path(video_path): axum::extract::Path<String>,
    req: axum::extract::Request<axum::body::Body>,
) -> impl IntoResponse {
    let safe_path = match std::fs::canonicalize(state.config.data_dir.join(&video_path)) {
        Ok(path) => path,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::NOT_FOUND,
                format!("{}文件或文件夹不存在", video_path),
            )
                .into_response();
        }
    };

    if safe_path.is_dir() {
        error!("{}不是文件夹", safe_path.display());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}不是文件夹", video_path),
        )
            .into_response();
    }

    match tower_http::services::ServeFile::new(safe_path)
        .oneshot(req)
        .await
    {
        Ok(res) => res.into_response(),
        Err(err) => {
            error!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
