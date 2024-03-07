use core::fmt;

use serde::Deserialize;

use crate::macros::strong_alias;
pub use crate::strong::Strong;

strong_alias!(ServerUrl, String, Debug, PartialEq, Eq);
strong_alias!(Username, String, Debug, PartialEq, Eq);
strong_alias!(Password, String, Debug, PartialEq, Eq);
strong_alias!(PasswordHash, String, Debug, PartialEq, Eq);
strong_alias!(Salt, String, Debug, PartialEq, Eq);

strong_alias!(AlbumId, String, Debug, PartialEq, Eq);
strong_alias!(ArtistId, String, Debug, PartialEq, Eq);
strong_alias!(MusicFolderId, String, Debug, PartialEq, Eq);
