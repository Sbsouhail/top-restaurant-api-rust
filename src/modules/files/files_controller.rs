use std::sync::Arc;

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use chrono::Utc;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{modules::shared::shared_dto::AppResult, AppState};

use super::files_dto::FileUploaded;
// use futures_util::stream::StreamExt;

pub async fn upload(
    State(_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> AppResult<FileUploaded> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "image" {
            let data = field.bytes().await.unwrap();

            let image_name: i64 = Utc::now().timestamp();

            let mut file = File::create(format!("./public/uploads/{}.jpeg", image_name))
                .await
                .unwrap();

            file.write(&data).await.unwrap();

            return AppResult::Result(
                StatusCode::CREATED,
                FileUploaded {
                    path: format!("api/files/uploads/{}.jpeg", image_name),
                },
            );
        }
    }
    return AppResult::Error(StatusCode::BAD_REQUEST, "image is required".to_string());
}
