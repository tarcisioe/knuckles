use rand::{distributions::DistString, rngs::ThreadRng, Rng};

use crate::{
    token::TokenInfo,
    types::{Password, PasswordHash, Salt},
};

pub fn salted_md5(password: &Password, salt: &Salt) -> PasswordHash {
    let payload = format!("{password}{salt}");
    let hash = md5::compute(payload);

    PasswordHash(format!("{:x}", hash))
}

pub struct Hasher<R: Rng> {
    rng: R,
}

impl<R: Rng> Hasher<R> {
    fn new(rng: R) -> Self {
        Self { rng }
    }

    fn salt(&mut self) -> Salt {
        Salt(rand::distributions::Alphanumeric.sample_string(&mut self.rng, 7))
    }

    pub fn md5_with_random_salt(&mut self, password: &Password) -> TokenInfo {
        let salt = self.salt();

        TokenInfo {
            hash: salted_md5(password, &salt),
            salt,
        }
    }
}

pub fn default_hasher() -> Hasher<ThreadRng> {
    Hasher::new(rand::thread_rng())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rand::SeedableRng;

    use crate::types::Strong;

    use super::*;

    #[test]
    fn test_salt_generation() -> Result<()> {
        let rng = rand::rngs::StdRng::seed_from_u64(10);
        let mut hasher = Hasher::new(rng);

        assert_eq!(hasher.salt().get(), "YIVLnWx");

        Ok(())
    }

    #[test]
    fn test_salted_md5() -> Result<()> {
        let hash = salted_md5(
            &Password::unchecked("testpassword"),
            &Salt::unchecked("YIVLnWx"),
        );

        assert_eq!(hash.get(), "1b7c28b40f08a05b377fc6a8dda3beea");

        Ok(())
    }

    #[test]
    fn test_md5_with_random_salt() -> Result<()> {
        let rng = rand::rngs::StdRng::seed_from_u64(10);
        let mut hasher = Hasher::new(rng);

        let token_info = hasher.md5_with_random_salt(&Password::unchecked("testpassword"));

        assert_eq!(token_info.hash.get(), "1b7c28b40f08a05b377fc6a8dda3beea");
        assert_eq!(token_info.salt.get(), "YIVLnWx");

        Ok(())
    }
}
