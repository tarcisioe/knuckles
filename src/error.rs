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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_missing() {
        let a: Option<i64> = None;

        match a.on_missing("a") {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), "Missing optional attribute a."),
        }

        let a = Some(42);

        match a.on_missing("a") {
            Ok(n) => assert_eq!(n, 42),
            Err(_) => assert!(false),
        }
    }
}
