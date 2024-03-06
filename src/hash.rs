use rand::distributions::DistString;

use crate::{token::TokenInfo, types::{Password, PasswordHash, Salt}};

pub fn salt() -> Salt {
    Salt(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 7))
}

pub fn salted_md5(password: &Password, salt: &Salt) -> PasswordHash {
    let payload = format!("{password}{salt}");
    let hash = md5::compute(payload);

    PasswordHash(format!("{:x}", hash))
}

pub fn md5_with_random_salt(password: &Password) -> TokenInfo {
    let salt = salt();

    TokenInfo { hash: salted_md5(&password, &salt), salt }
}
