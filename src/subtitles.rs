//! GET: https://api.opensubtitles.com/api/v1/subtitles

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtitles {
    pub total_pages: Option<i64>,
    pub total_count: Option<i64>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub data: Option<Vec<Datum>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datum {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub datum_type: Option<String>,
    pub attributes: Option<Attributes>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub subtitle_id: Option<String>,
    pub language: Option<String>,
    pub download_count: Option<i64>,
    pub new_download_count: Option<i64>,
    pub hearing_impaired: Option<bool>,
    pub hd: Option<bool>,
    pub fps: Option<f64>,
    pub votes: Option<i64>,
    pub ratings: Option<f64>,
    pub from_trusted: Option<bool>,
    pub foreign_parts_only: Option<bool>,
    pub upload_date: Option<String>,
    pub ai_translated: Option<bool>,
    pub nb_cd: Option<i64>,
    pub slug: Option<String>,
    pub machine_translated: Option<bool>,
    pub release: Option<String>,
    pub comments: Option<String>,
    pub legacy_subtitle_id: Option<i64>,
    pub legacy_uploader_id: Option<i64>,
    pub uploader: Option<Uploader>,
    pub feature_details: Option<FeatureDetails>,
    pub url: Option<String>,
    pub related_links: Option<Vec<RelatedLink>>,
    pub files: Option<Vec<File>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureDetails {
    pub feature_id: Option<i64>,
    pub feature_type: Option<String>,
    pub year: Option<i64>,
    pub title: Option<String>,
    pub movie_name: Option<String>,
    pub imdb_id: Option<i64>,
    pub tmdb_id: Option<i64>,
    pub season_number: Option<i64>,
    pub episode_number: Option<i64>,
    pub parent_imdb_id: Option<i64>,
    pub parent_title: Option<String>,
    pub parent_tmdb_id: Option<i64>,
    pub parent_feature_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub file_id: Option<i64>,
    pub cd_number: Option<i64>,
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedLink {
    pub label: Option<String>,
    pub url: Option<String>,
    pub img_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uploader {
    pub uploader_id: Option<i64>,
    pub name: Option<String>,
    pub rank: Option<String>,
}
