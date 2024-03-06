use core::fmt;

use serde::Deserialize;

use crate::macros::strong_string;

strong_string!(ServerUrl);
strong_string!(Username);
strong_string!(Password);
strong_string!(PasswordHash);
strong_string!(Salt);
