use std::io::BufReader;

use anyhow::Result;

mod api_types;
mod client;
mod config;
mod error;
mod hash;
mod macros;
mod stream;
mod strong;
#[cfg(test)]
mod test_util;
mod token;
mod types;

use hash::default_hasher;

use crate::{
    config::{default_config_file_path, make_client, read_config_from_path},
    types::SongId,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = default_config_file_path()?;
    let config = read_config_from_path(&config_path)?;
    let client = make_client(&config, &mut default_hasher());

    /*
    dbg!(client.ping().await?);

    let r = client.albums(AlbumListType::AlphabeticalByName, None, None, None).await?;

    dbg!(r);

    let r = client.album(&AlbumId::unchecked("69b7b8e47762e9a9fdc6ac558003ca49")).await?;

    dbg!(r);
    */

    let r = client
        .stream(&SongId::unchecked("8ff72dd73e11810de0675cba67cf4a4f"))
        .await?;

    let song_stream = stream::from_response(r);

    let buffered = BufReader::new(song_stream);

    let decoder = tokio::task::block_in_place(|| rodio::decoder::Decoder::new(buffered))?;

    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&stream_handle)?;

    sink.append(decoder);

    sink.sleep_until_end();

    Ok(())
}
