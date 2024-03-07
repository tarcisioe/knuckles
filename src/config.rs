use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rand::Rng;
use serde::Deserialize;

use crate::client::SubsonicClient;
use crate::hash::Hasher;
use crate::token::TokenInfo;
use crate::types::{Password, ServerUrl, Username};

const DEFAULT_CONFIG_FILENAME: &str = "knuckles.toml";

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuthInfo {
    Password(Password),
    Token(TokenInfo),
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct SubsonicConfig {
    url: ServerUrl,
    username: Username,
    #[serde(flatten)]
    auth_info: AuthInfo,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub client: SubsonicConfig,
}

pub fn make_candidate_config_path(base: &Path) -> PathBuf {
    base.join(DEFAULT_CONFIG_FILENAME)
}

#[cfg(not(tarpaulin_include))]
pub fn default_config_directory() -> Result<PathBuf> {
    Ok(dirs::config_dir().context("Could not find a configuration directory for this platform.")?)
}

#[cfg(not(tarpaulin_include))]
pub fn default_config_file_path() -> Result<PathBuf> {
    Ok(make_candidate_config_path(&default_config_directory()?))
}

pub fn read_config_from_string(config: &str) -> Result<Config> {
    Ok(toml::from_str(config)?)
}

pub fn read_config_from_path(config_path: &Path) -> Result<Config> {
    read_config_from_string(&fs::read_to_string(config_path)?)
}

pub fn make_client<R: Rng>(config: &Config, hasher: &mut Hasher<R>) -> SubsonicClient {
    use AuthInfo::*;

    let token_info = match &config.client.auth_info {
        Password(password) => hasher.md5_with_random_salt(password),
        Token(token_info) => token_info.clone(),
    };

    SubsonicClient {
        url: config.client.url.clone(),
        username: config.client.username.clone(),
        token_info,
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rand::SeedableRng;
    use textwrap::dedent;

    use crate::{test_util::test_data_path, types::{PasswordHash, Salt}};

    use super::*;

    #[test]
    fn test_read_well_formed_config_with_password() -> Result<()> {
        let config_text = dedent(r#"
            [client]
            url = "dummyurl"
            username = "test"
            password = "password"
        "#);

        let config = read_config_from_string(&config_text)?;

        let expected = Config {
            client: SubsonicConfig {
               url: ServerUrl::unchecked("dummyurl") ,
               username: Username::unchecked("test") ,
               auth_info: AuthInfo::Password(Password::unchecked("password")),
            }
        };

        assert_eq!(config, expected);

        Ok(())
    }

    #[test]
    fn test_read_well_formed_config_with_hash_and_salt() -> Result<()> {
        let config_text = dedent(r#"
            [client]
            url = "dummyurl"
            username = "test"

            [client.token]
            hash = "a1b2c3"
            salt = "abcde"
        "#);

        let config = read_config_from_string(&config_text)?;

        let expected = Config {
            client: SubsonicConfig {
               url: ServerUrl::unchecked("dummyurl") ,
               username: Username::unchecked("test") ,
               auth_info: AuthInfo::Token(TokenInfo{
                   hash: PasswordHash::unchecked("a1b2c3"),
                   salt: Salt::unchecked("abcde"),
               }),
            }
        };

        assert_eq!(config, expected);

        Ok(())
    }

    #[test]
    fn test_make_client_with_config_password() -> Result<()> {
        let rng = rand::rngs::StdRng::seed_from_u64(10);
        let mut hasher = Hasher::new(rng);

        let config_text = dedent(r#"
            [client]
            url = "dummyurl"
            username = "test"
            password = "password"
        "#);

        let config = read_config_from_string(&config_text)?;

        let client = make_client(&config, &mut hasher);

        let expected = SubsonicClient {
            url: ServerUrl::unchecked("dummyurl"),
            username: Username::unchecked("test"),
            token_info: TokenInfo {
                hash: PasswordHash::unchecked("cc4574efec464ba75cce2c1c36a6e028"),
                salt: Salt::unchecked("YIVLnWx"),
            }
        };

        assert_eq!(client, expected);

        Ok(())
    }

    #[test]
    fn test_make_client_with_config_token() -> Result<()> {
        let rng = rand::rngs::StdRng::seed_from_u64(10);
        let mut hasher = Hasher::new(rng);

        let config_text = dedent(r#"
            [client]
            url = "dummyurl"
            username = "test"

            [client.token]
            hash = "a1b2c3"
            salt = "abcde"
        "#);

        let config = read_config_from_string(&config_text)?;

        let client = make_client(&config, &mut hasher);

        let expected = SubsonicClient {
            url: ServerUrl::unchecked("dummyurl"),
            username: Username::unchecked("test"),
            token_info: TokenInfo {
                hash: PasswordHash::unchecked("a1b2c3"),
                salt: Salt::unchecked("abcde"),
            }
        };

        assert_eq!(client, expected);

        Ok(())
    }

    #[test]
    fn test_make_candidate_config_path() {
        assert_eq!(make_candidate_config_path(Path::new("/path/to/configs")), Path::new("/path/to/configs/knuckles.toml"));
    }

    #[test]
    fn test_read_config_from_path_with_token() -> Result<()> {
        let config_path = test_data_path("test_read_config_from_path/knuckles-token.toml");

        let config = read_config_from_path(&config_path)?;

        let expected = Config {
            client: SubsonicConfig {
               url: ServerUrl::unchecked("dummyurl") ,
               username: Username::unchecked("test") ,
               auth_info: AuthInfo::Token(TokenInfo{
                   hash: PasswordHash::unchecked("a1b2c3"),
                   salt: Salt::unchecked("abcde"),
               }),
            }
        };

        assert_eq!(config, expected);

        Ok(())
    }

    #[test]
    fn test_read_config_from_path_with_password() -> Result<()> {
        let config_path = test_data_path("test_read_config_from_path/knuckles-password.toml");

        let config = read_config_from_path(&config_path)?;

        let expected = Config {
            client: SubsonicConfig {
               url: ServerUrl::unchecked("dummyurl") ,
               username: Username::unchecked("test") ,
               auth_info: AuthInfo::Password(Password::unchecked("password")),
            }
        };

        assert_eq!(config, expected);

        Ok(())
    }
}
