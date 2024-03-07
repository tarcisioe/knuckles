use std::path::{Path, PathBuf};

fn test_datadir_base() -> PathBuf {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    base.push("resources/tests/");

    base
}

pub fn test_data_path(file: impl AsRef<Path>) -> PathBuf {
    let mut base = test_datadir_base();

    base.push(file);

    base
}
