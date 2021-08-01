use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::details::NekosDetails;

/// A response from the api
#[derive(serde::Deserialize, Debug, Clone, Hash)]
#[serde(transparent)]
pub struct NekosBestResponse {
    /// The list of urls returned, with artist and source details if
    /// using [`Category::Nekos`]
    #[serde(deserialize_with = "serde_utils::response_or_seq_response")]
    pub url: Vec<NekosBestResponseSingle>,
}

impl Index<usize> for NekosBestResponse {
    type Output = NekosBestResponseSingle;

    fn index(&self, index: usize) -> &Self::Output {
        self.url.index(index)
    }
}

impl IndexMut<usize> for NekosBestResponse {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.url.index_mut(index)
    }
}

impl Deref for NekosBestResponse {
    type Target = Vec<NekosBestResponseSingle>;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl DerefMut for NekosBestResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.url
    }
}

/// A response from the api, in the case of requesting a single
/// url with [`get`] or [`get_with_client`]
#[derive(Debug, Clone, Hash, serde::Deserialize)]
pub struct NekosBestResponseSingle {
    /// The url
    pub url: String,
    /// The details, in case of [`Category::Nekos`]
    #[serde(flatten, default)]
    pub details: Option<NekosDetails>,
}

impl Deref for NekosBestResponseSingle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl DerefMut for NekosBestResponseSingle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.url
    }
}

mod serde_utils {
    // serde helpers
    use std::fmt;

    use serde::{de, Deserialize, Deserializer};

    use super::NekosBestResponseSingle;

    pub fn response_or_seq_response<'de, D>(
        deserializer: D,
    ) -> Result<Vec<NekosBestResponseSingle>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResponseSingleOrVec;

        impl<'de> de::Visitor<'de> for ResponseSingleOrVec {
            type Value = Vec<NekosBestResponseSingle>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("nekos details or list of nekos details")
            }

            fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
                    .map(|it| vec![it])
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![])
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(self)
            }
        }

        deserializer.deserialize_option(ResponseSingleOrVec)
    }
}
