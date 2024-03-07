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

use crate::{config::{default_config_file_path, make_client, read_config_from_path}, hash::default_hasher};

fn main() -> Result<()> {
    let config_path = default_config_file_path()?;
    let config = read_config_from_path(&config_path)?;
    let client = make_client(&config, &mut default_hasher());

    dbg!(client.ping()?);

    let r = client.albums()?;

    dbg!(r);

    Ok(())
}
