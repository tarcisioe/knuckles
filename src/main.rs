use anyhow::Result;

mod api_types;
mod client;
mod config;
mod error;
mod hash;
mod macros;
mod strong;
#[cfg(test)]
mod test_util;
mod token;
mod types;

use crate::{
    client::AlbumListType,
    config::{default_config_file_path, make_client, read_config_from_path},
    hash::default_hasher,
    types::{AlbumId, SongId},
};

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = default_config_file_path()?;
    let config = read_config_from_path(&config_path)?;
    let client = make_client(&config, &mut default_hasher());

    dbg!(client.ping().await?);

    let r = client.albums(AlbumListType::AlphabeticalByName, None, None, None).await?;

    dbg!(r);

    let r = client.album(&AlbumId::unchecked("69b7b8e47762e9a9fdc6ac558003ca49")).await?;

    dbg!(r);

    /*
    let r = client.stream(&SongId::unchecked("8ff72dd73e11810de0675cba67cf4a4f")).await?;
    */

    Ok(())
}
