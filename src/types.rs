use core::fmt;

use serde::Deserialize;

use crate::macros::strong_string;

pub trait Strong<T> {
    fn get(&self) -> &T;
}

strong_string!(ServerUrl);
strong_string!(Username);
strong_string!(Password);
strong_string!(PasswordHash);
strong_string!(Salt);
