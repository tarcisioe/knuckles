use core::fmt;

use serde::Deserialize;

use crate::macros::strong_alias;

pub trait Strong<T> {
    fn get(&self) -> &T;
}

strong_alias!(ServerUrl, String);
strong_alias!(Username, String);
strong_alias!(Password, String);
strong_alias!(PasswordHash, String);
strong_alias!(Salt, String);
