use std::{fs, sync::Arc};

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

            let res = file.write(&data).await;

            return match res {
                Ok(_) => AppResult::Result(
                    StatusCode::CREATED,
                    FileUploaded {
                        path: format!("api/files/uploads/{}.jpeg", image_name),
                    },
                ),
                Err(_) => AppResult::Error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong!".to_string(),
                ),
            };
        }
    }
    return AppResult::Error(StatusCode::BAD_REQUEST, "image is required".to_string());
}

pub fn delete_file(api_uri: String) {
    // Assuming your project root is the current working directory
    let project_root = std::env::current_dir().expect("Failed to get current directory");

    // Construct the full path by appending the relative path to the project root
    let full_path = project_root
        .join("public")
        .join(api_uri.trim_start_matches("api/files/"));

    // Attempt to remove the file
    if let Err(err) = fs::remove_file(&full_path) {
        eprintln!("Error deleting file: {:?}", err);
    } else {
        println!("File deleted successfully");
    }
}
