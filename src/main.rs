use anyhow::Result;

mod api_types;
mod client;
mod config;
mod error;
mod hash;
mod macros;
mod token;
mod types;

use crate::config::{make_client, read_config};

fn main() -> Result<()> {
    let client = make_client(&read_config()?);

    dbg!(client.ping()?);

    let r = client.albums()?;

    dbg!(r);

    Ok(())
}
