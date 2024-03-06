use serde::Deserialize;

use crate::types::{PasswordHash, Salt};

#[derive(Clone, Deserialize)]
pub struct TokenInfo {
    pub hash: PasswordHash,
    pub salt: Salt,
}
