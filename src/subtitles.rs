//! GET: https://api.opensubtitles.com/api/v1/subtitles

serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitles {
    pub total_pages: i64,
    pub total_count: i64,
    pub per_page: i64,
    pub page: i64,
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub subtitle_id: String,
    pub language: String,
    pub download_count: i64,
    pub new_download_count: i64,
    pub hearing_impaired: bool,
    pub hd: bool,
    pub fps: i64,
    pub votes: i64,
    pub ratings: i64,
    pub from_trusted: bool,
    pub foreign_parts_only: bool,
    pub upload_date: String,
    pub ai_translated: bool,
    pub nb_cd: i64,
    pub slug: String,
    pub machine_translated: bool,
    pub release: String,
    pub comments: String,
    pub legacy_subtitle_id: i64,
    pub legacy_uploader_id: i64,
    pub uploader: Uploader,
    pub feature_details: FeatureDetails,
    pub url: String,
    pub related_links: Vec<RelatedLink>,
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDetails {
    pub feature_id: i64,
    pub feature_type: String,
    pub year: i64,
    pub title: String,
    pub movie_name: String,
    pub imdb_id: i64,
    pub tmdb_id: i64,
    pub season_number: i64,
    pub episode_number: i64,
    pub parent_imdb_id: i64,
    pub parent_title: String,
    pub parent_tmdb_id: i64,
    pub parent_feature_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub file_id: i64,
    pub cd_number: i64,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedLink {
    pub label: String,
    pub url: String,
    pub img_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uploader {
    pub uploader_id: i64,
    pub name: String,
    pub rank: String,
}

