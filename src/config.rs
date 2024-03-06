use std::fs;
use std::path::PathBuf;

use anyhow::{Result, Context};
use serde::Deserialize;

use crate::client::SubsonicClient;
use crate::hash::md5_with_random_salt;
use crate::token::TokenInfo;
use crate::types::{Password, ServerUrl, Username};

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
pub enum AuthInfo {
    Password(Password),
    Token(TokenInfo),
}

#[derive(Deserialize)]
pub struct SubsonicConfig {
    url: ServerUrl,
    username: Username,
    #[serde(flatten)]
    auth_info: AuthInfo,
}

#[derive(Deserialize)]
pub struct Config {
    pub client: SubsonicConfig,
}


pub fn read_config() -> Result<Config> {
    let config_path: PathBuf = [
        dirs::config_dir()
            .context("Could not find a configuration directory for this platform.")?,
        "knuckles.toml".into(),
    ].iter().collect();

    Ok(toml::from_str(&fs::read_to_string(config_path)?)?)
}


pub fn make_client(config: &Config) -> SubsonicClient {
    use AuthInfo::*;

    let token_info = match &config.client.auth_info {
        Password(password) => md5_with_random_salt(&password),
        Token(token_info) => token_info.clone(),
    };

    SubsonicClient {
        url: config.client.url.clone(),
        username: config.client.username.clone(),
        token_info,
    }
}
