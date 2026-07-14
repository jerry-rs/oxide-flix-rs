use crate::state::AppState;
use axum::response::IntoResponse;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

pub(crate) async fn video_upload_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    upload_path: Option<axum::extract::Path<String>>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let video_upload_path = match upload_path {
        Some(axum::extract::Path(u_path)) => {
            match std::fs::canonicalize(state.config.data_dir.join(u_path)) {
                Ok(p) => p,
                Err(e) => {
                    error!("{e}");
                    state.config.data_dir.clone()
                }
            }
        }
        None => state.config.data_dir.clone(),
    };


    while let Ok(Some(mut field)) = multipart.next_field().await{

        let file_name = field.file_name()
            .unwrap_or_default()
            .to_string();
        info!("Uploading {} to: {}",&file_name,video_upload_path.display());
        let file_path = video_upload_path.join(file_name);
        match tokio::fs::File::create(&file_path).await{
            Ok(mut file) => {
                loop {
                    match field.chunk().await{
                        Ok(Some(chunk)) => {
                            if let Err(e) = file.write_all(&chunk).await{
                                error!("{e}");
                                break;
                            }
                        },
                        Ok(None) => {
                            break;
                        },
                        Err(e) => {
                            error!("{e}");
                            break;
                        }
                    }
                }
            },
            Err(e) => {
                error!("{e}");
                continue;
            }
        }
        info!("Success to Upload {}",file_path.display());
    }
}
