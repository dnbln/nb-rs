//! Details of an [`crate::Category::Nekos`] image.
//! Refer to [`NekosDetails`].

use serde::{Deserialize, Deserializer};
use url::Url;

/// In the case of [`Category::Nekos`], the API
/// also returns the source url, the name and a
/// link to the artist that made it.
#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NekoDetails {
    #[serde(deserialize_with = "deserialize_url")]
    pub artist_href: Url,
    pub artist_name: String,
    #[serde(deserialize_with = "deserialize_url")]
    pub source_url: Url,
}

fn deserialize_url<'de, D: Deserializer<'de>>(de: D) -> Result<Url, D::Error> {
    let s = String::deserialize(de)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}

#[deprecated(since = "0.11.0", note = "Use `NekoDetails` instead")]
pub type NekosDetails = NekoDetails;

/// In the case of gif endpoints, the API also
/// returns the anime name.
#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GifDetails {
    pub anime_name: String,
}

#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Details {
    // #[serde(flatten)]
    Neko(NekoDetails),
    // #[serde(flatten)]
    Gif(GifDetails),
}

impl Details {
    /// Returns `true` if the details is [`Neko`].
    ///
    /// [`Neko`]: Details::Neko
    pub fn is_nekos(&self) -> bool {
        matches!(self, Self::Neko(..))
    }

    pub fn as_neko(&self) -> Option<&NekoDetails> {
        if let Self::Neko(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_neko(self) -> Result<NekoDetails, Self> {
        if let Self::Neko(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the details is [`Gif`].
    ///
    /// [`Gif`]: Details::Gif
    pub fn is_gif(&self) -> bool {
        matches!(self, Self::Gif(..))
    }

    pub fn as_gif(&self) -> Option<&GifDetails> {
        if let Self::Gif(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_gif(self) -> Result<GifDetails, Self> {
        if let Self::Gif(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl From<NekoDetails> for Details {
    fn from(v: NekoDetails) -> Self {
        Self::Neko(v)
    }
}

impl From<GifDetails> for Details {
    fn from(v: GifDetails) -> Self {
        Self::Gif(v)
    }
}
