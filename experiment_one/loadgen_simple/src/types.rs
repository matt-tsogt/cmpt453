use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SimpleRequestJson {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SimpleResponseJson {
    pub message: String,
    pub timestamp: i64,
}