use serde::Serialize;

#[derive(Serialize)]
pub struct FileUploaded {
    pub path: String,
}
