use serde::Deserialize;

use crate::types::{PasswordHash, Salt};

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct TokenInfo {
    pub hash: PasswordHash,
    pub salt: Salt,
}
