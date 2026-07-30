use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Download {
    pub link: String,
    pub file_name: String,
    pub requests: u64,
    pub remaining: u64,
    pub message: String,
    pub reset_time: String,
    pub reset_time_utc: String,
}
