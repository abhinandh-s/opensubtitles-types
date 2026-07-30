//! GET: https://api.opensubtitles.com/api/v1/subtitles

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtitles {
    pub total_pages: u64,
    pub total_count: u64,
    pub per_page: u64,
    pub page: u64,
    #[serde(default)]
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub subtitle_id: String,
    pub language: String,
    #[serde(default)]
    pub download_count: u64,
    #[serde(default)]
    pub new_download_count: u64,
    #[serde(default)]
    pub hearing_impaired: bool,
    #[serde(default)]
    pub hd: bool,
    pub fps: Option<f64>,
    pub votes: Option<u64>,
    pub ratings: Option<f64>,
    #[serde(default)]
    pub from_trusted: bool,
    #[serde(default)]
    pub foreign_parts_only: bool,
    pub upload_date: String,
    #[serde(default)]
    pub ai_translated: bool,
    #[serde(default)]
    pub nb_cd: u64,
    pub slug: String,
    #[serde(default)]
    pub machine_translated: bool,
    pub release: Option<String>,
    pub comments: Option<String>,
    pub legacy_subtitle_id: Option<u64>,
    pub legacy_uploader_id: Option<u64>,
    pub uploader: Uploader,
    pub feature_details: FeatureDetails,
    pub url: String,
    #[serde(default)]
    pub related_links: Vec<RelatedLink>,
    #[serde(default)]
    pub files: Vec<File>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureDetails {
    pub feature_id: u64,
    pub feature_type: String,
    pub year: u64,
    pub title: String,
    pub movie_name: String,
    pub imdb_id: Option<u64>,
    pub tmdb_id: Option<u64>,
    pub season_number: Option<u64>,
    pub episode_number: Option<u64>,
    pub parent_imdb_id: Option<u64>,
    pub parent_title: Option<String>,
    pub parent_tmdb_id: Option<u64>,
    pub parent_feature_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub file_id: u64,
    pub cd_number: u64,
    pub file_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedLink {
    pub label: String,
    pub url: String,
    pub img_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uploader {
    pub uploader_id: Option<u64>, // What about Anonymous uploader, let keep it wrapped in Option
    pub name: String,
    pub rank: String,
}
