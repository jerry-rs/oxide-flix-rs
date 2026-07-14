use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::{error, info};

pub(crate) async fn video_delete_handler(
    State(state): State<AppState>,
    axum::extract::Path(delete_path): axum::extract::Path<String>,
) -> Response {
    if delete_path.trim().is_empty() {
        error!("{delete_path} is root path and not delete");
        return (StatusCode::BAD_REQUEST, "not delete root path").into_response();
    }
    let safe_path = match std::fs::canonicalize(state.config.data_dir.join(&delete_path)) {
        Ok(path) => path,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::NOT_FOUND,
                format!("{} file or folder not exist", delete_path),
            )
                .into_response();
        }
    };
    if safe_path.is_dir() {
        if let Err(e) = tokio::fs::remove_dir_all(&safe_path).await {
            error!("failed to delete {} :{e}", &safe_path.display());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to delete {}", delete_path),
            )
                .into_response();
        }
    } else {
        if let Err(e) = tokio::fs::remove_file(&safe_path).await {
            error!("failed to delete {} :{e}", &safe_path.display());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to delete {}", delete_path),
            )
                .into_response();
        }
    };
    info!("success to delete {}", safe_path.display());
    (StatusCode::OK, "SUCCESS").into_response()
}
