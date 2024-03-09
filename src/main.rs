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

use client::AlbumListType;
use hash::default_hasher;
use stream::{SongStream, SyncReader};

use crate::config::{default_config_file_path, make_client, read_config_from_path};

fn play_stream(s: SongStream<SyncReader>) -> Result<()> {
    let buffered = BufReader::new(s);

    let decoder = tokio::task::block_in_place(|| rodio::decoder::Decoder::new(buffered))?;

    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&stream_handle)?;

    sink.append(decoder);

    sink.sleep_until_end();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = default_config_file_path()?;
    let config = read_config_from_path(&config_path)?;
    let client = make_client(&config, &mut default_hasher());

    dbg!(client.ping().await?);

    let albums = client
        .albums(AlbumListType::AlphabeticalByName, None, None, None)
        .await?;
    let first_album = client.album(&albums[0].id).await?;
    let first_song = client.stream(&first_album.song[0].id, Some(true)).await?;

    let song_stream = stream::from_response(first_song);

    play_stream(song_stream)
}
