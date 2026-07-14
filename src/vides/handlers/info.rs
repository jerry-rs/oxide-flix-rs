use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tracing::error;

// 真实视频元数据返回结构
#[derive(Serialize)]
struct VideoInfoResponse {
    filename: String,
    size_bytes: u64,
    mime_type: String,
    sha256: String,
    // 以下字段在真实生产环境中，建议在视频上传/扫描时利用 tokio::process::Command 调用 ffprobe 解析并存入数据库
    // 或者使用纯 Rust 媒体解析库（如 symphonia 或 mp4 轮子）动态读取
    suggested_player_mode: String,
}

pub(crate) async fn video_info_handler(
    State(state): State<AppState>,
    axum::extract::Path(video_path): axum::extract::Path<String>,
) -> impl IntoResponse {
    // 同样使用安全路径校验
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
    // 读取真实的底层文件系统元数据
    let metadata = match tokio::fs::metadata(&safe_path).await {
        Ok(meta) => meta,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("无法读取{}的meta信息", video_path),
            )
                .into_response();
        }
    };
    let filename = safe_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    // 自动识别视频的 MimeType（如 video/mp4, video/x-matroska 等），前端 HTML5 Player 需要这个参数
    let mime_type = mime_guess::from_path(&safe_path)
        .first_or_octet_stream()
        .to_string();

    let mut hash = ring::digest::Context::new(&ring::digest::SHA256);
    match tokio::fs::File::open(&safe_path).await {
        Ok(mut file) => {
            // 3. 分配一个 64KB 的固定缓冲区（内存开销极小且极其高效）
            let mut buffer = [0u8; 65536];

            loop {
                // 异步读取最多 64KB 的数据到缓冲区
                match file.read(&mut buffer).await {
                    Ok(0) => {
                        // 读取到 0 字节，说明文件已经完全读完（EOF），安全退出循环
                        break;
                    }
                    Ok(n) => {
                        // 仅将实际读取到的 n 字节数据喂给哈希生成器
                        hash.update(&buffer[..n]);
                    }
                    Err(e) => {
                        error!("读取文件流时出错: {e}");
                        break;
                    }
                }
            }
        },
        Err(e) => {
            error!("{e}");
        }
    }
    let info = VideoInfoResponse {
        filename,
        size_bytes: metadata.len(),
        mime_type,
        suggested_player_mode: "html5-video".to_string(),
        sha256: hex::encode(hash.finish()),
    };
    (StatusCode::OK, Json(info)).into_response()
}
