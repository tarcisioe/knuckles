use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::types::{AlbumId, ArtistId, SongId};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayGain {
    pub album_peak: Option<f64>,
    pub track_peak: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumListItem {
    pub id: AlbumId,

    // Required fields
    pub created: DateTime<Utc>,
    pub duration: u64,
    pub is_dir: bool,
    pub song_count: u64,
    pub title: String,

    // Optional fields
    pub album: Option<String>,
    pub artist: Option<String>,
    pub artist_id: Option<ArtistId>,
    pub bpm: Option<u64>,
    pub comment: Option<String>,
    pub cover_art: Option<String>,
    pub genres: Option<Vec<String>>,
    pub is_video: Option<bool>,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub replay_gain: Option<ReplayGain>,
    pub sort_name: Option<String>,
    pub year: Option<u64>,

    // Renamed fields
    #[serde(rename = "type")]
    pub media_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumList {
    pub album: Option<Vec<AlbumListItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumID3 {
    pub id: AlbumId,

    // Required fields
    pub created: DateTime<Utc>,
    pub duration: u64,
    pub name: String,
    pub song_count: u64,

    // Optional fields
    pub artist: Option<String>,
    pub artist_id: Option<ArtistId>,
    pub cover_art: Option<String>,
    pub genre: Option<String>,
    pub play_count: Option<u64>,
    pub starred: Option<DateTime<Utc>>,
    pub year: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: SongId,

    // Required fields
    pub is_dir: bool,
    pub title: String,

    // Optional fields
    pub album: Option<String>,
    pub album_id: Option<AlbumId>,
    pub artist: Option<String>,
    pub artist_id: Option<ArtistId>,
    pub bit_rate: Option<u64>,
    pub bpm: Option<u64>,
    pub comment: Option<String>,
    pub content_type: Option<String>,
    pub cover_art: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub disc_number: Option<u64>,
    pub duration: Option<u64>,
    pub genres: Option<Vec<Genre>>,
    pub is_video: Option<bool>,
    pub parent: Option<String>,
    pub path: Option<String>,
    pub replay_gain: Option<ReplayGain>,
    pub size: Option<u64>,
    pub sort_name: Option<String>,
    pub suffix: Option<String>,
    pub track: Option<u64>,
    pub year: Option<u64>,

    // Renamed fields
    #[serde(rename = "type")]
    pub media_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumID3WithSongs {
    #[serde(flatten)]
    pub album_data: AlbumID3,
    pub song: Vec<Song>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanStatus {
    pub scanning: bool,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResponse {
    // Required fields
    pub open_subsonic: bool,
    pub server_version: String,
    pub status: String,
    pub version: String,

    // Optional fields
    pub album: Option<AlbumID3WithSongs>,
    pub album_list: Option<AlbumList>,
    pub scan_status: Option<ScanStatus>,

    // Renamed fields
    #[serde(rename = "type")]
    pub server_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OuterSubsonicResponse {
    pub subsonic_response: SubsonicResponse,
}
