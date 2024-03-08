use core::fmt;

use anyhow::Result;
use reqwest::Url;

use crate::api_types::{AlbumID3WithSongs, AlbumListItem, OuterSubsonicResponse, SubsonicResponse};
use crate::error::OnMissing;
use crate::token::TokenInfo;
use crate::types::{AlbumId, MusicFolderId, ServerUrl, Strong, Username};

#[derive(Debug, PartialEq, Eq)]
pub struct SubsonicClient {
    pub url: ServerUrl,
    pub username: Username,
    pub token_info: TokenInfo,
}

trait WriteToUrl {
    fn write_to_url(&self, url: &mut Url);
}

#[allow(dead_code)]
pub enum AlbumListType {
    Random,
    Newest,
    Highest,
    Frequent,
    Recent,
    AlphabeticalByName,
    AlphabeticalByArtist,
    Starred,
    ByYear { from_year: String, to_year: String },
    ByGenre(String),
}

impl fmt::Display for AlbumListType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use AlbumListType::*;
        let s = match &self {
            Random => "random",
            Newest => "newest",
            Highest => "highest",
            Frequent => "frequent",
            Recent => "recent",
            AlphabeticalByName => "alphabeticalByName",
            AlphabeticalByArtist => "alphabeticalByArtist",
            Starred => "starred",
            ByYear { .. } => "byYear",
            ByGenre(_) => "byGenre",
        };

        fmt.write_str(s)
    }
}

impl WriteToUrl for AlbumListType {
    fn write_to_url(&self, url: &mut Url) {
        let mut qp = url.query_pairs_mut();

        match &self {
            AlbumListType::ByYear { from_year, to_year } => {
                qp.append_pair("fromYear", from_year);
                qp.append_pair("toYear", to_year);
            }
            AlbumListType::ByGenre(genre) => {
                qp.append_pair("genre", genre);
            }
            _ => {}
        }

        qp.append_pair("type", &self.to_string());
    }
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
            ("u", self.username.get_ref()),
            ("t", self.token_info.hash.get_ref()),
            ("s", self.token_info.salt.get_ref()),
            ("v", "1.16.1"),
            ("c", "knuckles"),
        ];

        let mut url = Url::parse_with_params(self.url.get_ref(), &params)?;

        url.set_path(&format!("rest/{path}"));

        Ok(url)
    }

    pub fn ping(&self) -> Result<SubsonicResponse> {
        subsonic_request(self.base_url("ping")?)
    }

    pub fn albums(
        &self,
        list_type: AlbumListType,
        size: Option<u64>,
        offset: Option<u64>,
        music_folder_id: Option<MusicFolderId>,
    ) -> Result<Vec<AlbumListItem>> {
        let mut url = self.base_url("getAlbumList")?;

        list_type.write_to_url(&mut url);

        if let Some(size) = size {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("size", &size.to_string());
        }

        if let Some(offset) = offset {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("offset", &offset.to_string());
        }

        if let Some(music_folder_id) = music_folder_id {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("offset", &music_folder_id.get());
        }

        let albums = subsonic_request(url)?
            .album_list
            .on_missing("album_list")?
            .album
            .unwrap_or_else(Vec::new);

        Ok(albums)
    }

    pub fn album(&self, id: &AlbumId) -> Result<AlbumID3WithSongs> {
        let mut url = self.base_url("getAlbum")?;

        url.query_pairs_mut().append_pair("id", id.get_ref());

        let albums = subsonic_request(url)?.album.on_missing("album")?;

        Ok(albums)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{PasswordHash, Salt};

    use super::*;

    #[test]
    fn test_base_url() -> Result<()> {
        let client = SubsonicClient {
            url: ServerUrl::unchecked("https://subsonic.example.com"),
            username: Username::unchecked("user"),
            token_info: TokenInfo {
                hash: PasswordHash::unchecked("a1b2c3"),
                salt: Salt::unchecked("abcde"),
            },
        };

        let base_url = client.base_url("ping")?;

        assert_eq!(base_url, Url::parse("https://subsonic.example.com/rest/ping?f=json&u=user&t=a1b2c3&s=abcde&v=1.16.1&c=knuckles")?);

        Ok(())
    }

    #[test]
    fn test_album_list_type_into_url() -> Result<()> {
        let base_url = Url::parse("https://subsonic.example.com/rest/getAlbumList")?;

        {
            let mut url = base_url.clone();

            AlbumListType::Random.write_to_url(&mut url);

            assert_eq!(
                url,
                Url::parse("https://subsonic.example.com/rest/getAlbumList?type=random")?,
            );
        }

        {
            let mut url = base_url.clone();

            AlbumListType::ByYear {
                from_year: "2019".to_owned(),
                to_year: "2022".to_owned(),
            }
            .write_to_url(&mut url);

            assert_eq!(
                url,
                Url::parse(
                    "https://subsonic.example.com/rest/getAlbumList?fromYear=2019&toYear=2022&type=byYear",
                )?,
            );
        }

        {
            let mut url = base_url.clone();

            AlbumListType::ByGenre("Heavy Metal".to_owned()).write_to_url(&mut url);

            assert_eq!(
                url,
                Url::parse(
                    "https://subsonic.example.com/rest/getAlbumList?genre=Heavy+Metal&type=byGenre",
                )?,
            );
        }

        Ok(())
    }
}
