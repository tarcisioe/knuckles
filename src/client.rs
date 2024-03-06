use anyhow::{Result, Context};
use reqwest::Url;

use crate::api_types::{AlbumListItem, OuterSubsonicResponse, SubsonicResponse};
use crate::error::missing_attribute;
use crate::token::TokenInfo;
use crate::types::{ServerUrl, Username};

pub struct SubsonicClient {
    pub url: ServerUrl,
    pub username: Username,
    pub token_info: TokenInfo,
}

fn raw_subsonic_request(url: Url) -> Result<OuterSubsonicResponse> {
    let json = reqwest::blocking::get(url)?.text()?;

    Ok(serde_json::from_str(&json)?)
}

fn subsonic_request(url: Url) -> Result<SubsonicResponse> {
    Ok(raw_subsonic_request(url)?.subsonic_response)
}

impl SubsonicClient {
    fn base_url(&self, path: &str) -> Result<Url> {
        let params = [
            ("f", "json"),
            ("u", self.username.as_ref()),
            ("t", self.token_info.hash.as_ref()),
            ("s", self.token_info.salt.as_ref()),
            ("v", "1.16.1"),
            ("c", "knuckles"),
        ];

        let mut url = Url::parse_with_params(&self.url.0, &params)?;

        url.set_path(&format!("rest/{path}"));

        Ok(url)
    }

    pub fn ping(&self) -> Result<SubsonicResponse> {
        subsonic_request(self.base_url("ping")?)
    }

    pub fn albums(&self) -> Result<Vec<AlbumListItem>> {
        let mut url = self.base_url("getAlbumList")?;

        url.query_pairs_mut()
            .append_pair("type", "alphabeticalByName");

        let albums = subsonic_request(url)?
                        .album_list.context(missing_attribute("album_list"))?
                        .album;

        Ok(albums)
    }
}

