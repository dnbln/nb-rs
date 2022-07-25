use serde::{Deserialize, Deserializer};
use url::Url;

/// In the case of [`Category::Neko`], the API
/// also returns the source url, the name and a
/// link to the artist that made it.
#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageDetails {
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
    Image(ImageDetails),
    Gif(GifDetails),
}

impl Details {
    /// Returns `true` if the details is [`Image`].
    ///
    /// [`Image`]: Details::Image
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(..))
    }

    pub fn as_image(&self) -> Option<&ImageDetails> {
        if let Self::Image(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_image(self) -> Result<ImageDetails, Self> {
        if let Self::Image(v) = self {
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

impl From<ImageDetails> for Details {
    fn from(v: ImageDetails) -> Self {
        Self::Image(v)
    }
}

impl From<GifDetails> for Details {
    fn from(v: GifDetails) -> Self {
        Self::Gif(v)
    }
}
