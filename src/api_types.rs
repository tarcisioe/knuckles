use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumListItem {
    pub id: String,
    pub is_dir: bool,
    pub song_count: u64,
    pub name: String,

    pub album: Option<String>,
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    pub bpm: Option<u64>,
    pub comment: Option<String>,
    pub cover_art: Option<String>,
    pub created: Option<DateTime<Utc>>, // maybe DateTime
    pub duration: Option<u64>,
    pub genres: Option<serde_json::Value>,
    pub is_video: Option<bool>,
    pub media_type: Option<String>,
    pub parent: Option<String>,
    pub replay_gain: Option<serde_json::Value>,
    pub sort_name: Option<String>,
    pub title: Option<String>,
    pub year: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumList {
    pub album: Vec<AlbumListItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResponse {
    pub open_subsonic: bool,
    pub server_version: String,
    pub status: String,
    #[serde(rename = "type")]
    pub server_type: String,
    pub version: String,
    pub album_list: Option<AlbumList>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OuterSubsonicResponse {
    pub subsonic_response: SubsonicResponse,
}
