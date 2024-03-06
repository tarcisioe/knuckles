use std::convert::Infallible;

use anyhow::{Context, Result};

pub trait OnMissing<T, E>: Context<T, E> {
    fn on_missing(self, attribute_name: &str) -> Result<T>;
}

impl<T> OnMissing<T, Infallible> for Option<T> {
    fn on_missing(self, attribute_name: &str) -> Result<T> {
        self.context(format!("Missing optional attribute {attribute_name}."))
    }
}
