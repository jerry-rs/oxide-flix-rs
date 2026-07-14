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
        error!("{delete_path}是根目录不允许删除");
        return (StatusCode::BAD_REQUEST, "不允许删除根目录").into_response();
    }
    let safe_path = match std::fs::canonicalize(state.config.data_dir.join(&delete_path)) {
        Ok(path) => path,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::NOT_FOUND,
                format!("{}文件或文件夹不存在", delete_path),
            )
                .into_response();
        }
    };
    if safe_path.is_dir() {
        if let Err(e) = tokio::fs::remove_dir_all(&safe_path).await {
            error!("失败删除文件夹 {} :{e}", &safe_path.display());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("失败删除文件夹 {}", delete_path),
            )
                .into_response();
        }
    } else {
        if let Err(e) = tokio::fs::remove_file(&safe_path).await {
            error!("失败删除文件 {} :{e}", &safe_path.display());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("失败删除文件 {}", delete_path),
            )
                .into_response();
        }
    };
    info!("成功删除{}",safe_path.display());
    (StatusCode::OK, "SUCCESS").into_response()
}
