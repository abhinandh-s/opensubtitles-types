use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub file_id: u64, // Required
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_format: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_fps: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_fps: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeshift: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_download: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub link: String,
    pub file_name: String,
    pub requests: u64,
    pub remaining: u64,
    pub message: String,
    pub reset_time: String,
    pub reset_time_utc: String,
}
