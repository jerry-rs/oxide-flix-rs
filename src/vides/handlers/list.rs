use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::{error};

#[derive(Serialize)]
struct DirEntry {
    name: String,
    is_dir: bool,
    size_bytes: u64,
}

fn is_video_ext(ext: &str) -> bool {
    matches!(
        ext.to_lowercase().as_str(),
        "mp4" | "mkv" | "webm" | "avi" | "mov" | "wmv" | "flv" | "m4v" | "ts" | "m3u8"
    )
}

#[derive(Serialize)]
struct ListDirResponse {
    entries: Vec<DirEntry>,
    current_path: String,
}

#[axum::debug_handler]
pub(crate) async fn video_list_handler(
    State(state): State<AppState>,
    list_path: Option<axum::extract::Path<String>>,
) -> Response {
    let safe_path = match list_path {
        Some(axum::extract::Path(l_path)) => {
            match std::fs::canonicalize(state.config.data_dir.join(&l_path)) {
                Ok(p) => p,
                Err(e) => {
                    error!("{e}");
                    state.config.data_dir.clone()
                }
            }
        }
        None => state.config.data_dir.clone(),
    };

    let mut entries = Vec::new();
    let mut read_dir = match tokio::fs::read_dir(&safe_path).await {
        Ok(rd) => rd,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    while let Some(entry) = read_dir.next_entry().await.unwrap_or_else(|_| None) {
        let name = entry.file_name().to_string_lossy().into_owned();
        // if name.starts_with('.') {
        //     continue;
        // }
        let is_dir = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
        let size_bytes = if is_dir {
            0
        } else {
            entry.metadata().await.map(|m| m.len()).unwrap_or(0)
        };
        // 只保留文件夹和视频文件
        if is_dir
            || is_video_ext(
                std::path::Path::new(&name)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or(""),
            )
        {
            entries.push(DirEntry {
                name,
                is_dir,
                size_bytes,
            });
        }
    }

    // 文件夹排在前面
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });

    let resp = ListDirResponse {
        entries,
        current_path: safe_path
            .to_string_lossy()
            .to_string()
            .trim_start_matches(
                state
                    .config
                    .data_dir
                    .clone()
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
            )
            .to_string(),
    };
    (StatusCode::OK, Json(resp)).into_response()
}
