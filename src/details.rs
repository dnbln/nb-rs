//! Details of an [`crate::Category::Nekos`] image.
//! Refer to [`NekosDetails`].

use serde::{Deserialize, Deserializer};

/// In the case of [`Category::Nekos`], the API
/// also returns the source url, the name and a
/// link to the artist that made it.
#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NekoDetails {
    pub artist_href: String,
    pub artist_name: String,
    pub source_url: String,
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
    Nekos(NekoDetails),
    // #[serde(flatten)]
    Gif(GifDetails),
}

impl Details {
    /// Returns `true` if the details is [`Nekos`].
    ///
    /// [`Nekos`]: Details::Nekos
    pub fn is_nekos(&self) -> bool {
        matches!(self, Self::Nekos(..))
    }

    pub fn as_nekos(&self) -> Option<&NekoDetails> {
        if let Self::Nekos(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_nekos(self) -> Result<NekoDetails, Self> {
        if let Self::Nekos(v) = self {
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
        Self::Nekos(v)
    }
}

impl From<GifDetails> for Details {
    fn from(v: GifDetails) -> Self {
        Self::Gif(v)
    }
}

use serde::de::Error;

fn decode_urlencoded<'de, De: Deserializer<'de>>(s: &str) -> Result<String, De::Error> {
    let s = urlencoding::decode(s).map_err(|e| Error::custom(e))?;
    let s = s.into_owned();

    Ok(s)
}

pub fn url_encoded_nekos_details_deserialize<'de, De: Deserializer<'de>>(
    de: De,
) -> Result<NekoDetails, De::Error> {
    #[derive(Deserialize)]
    struct Internal {
        artist_href: String,
        artist_name: String,
        source_url: String,
    }

    let internal = Internal::deserialize(de)?;

    let artist_href = decode_urlencoded::<De>(&internal.artist_href)?;
    let artist_name = decode_urlencoded::<De>(&internal.artist_name)?;
    let source_url = decode_urlencoded::<De>(&internal.source_url)?;

    Ok(NekoDetails {
        artist_href,
        artist_name,
        source_url,
    })
}

pub fn url_encoded_gif_details_deserialize<'de, De: Deserializer<'de>>(
    de: De,
) -> Result<GifDetails, De::Error> {
    #[derive(Deserialize)]
    struct Internal {
        anime_name: String,
    }

    let internal = Internal::deserialize(de)?;

    let anime_name = decode_urlencoded::<De>(&internal.anime_name)?;

    Ok(GifDetails { anime_name })
}
