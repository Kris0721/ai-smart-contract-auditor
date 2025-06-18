use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedCode {
    pub filename: String,
    pub content: String,
}


