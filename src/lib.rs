//! API wrapper for [nekos.best](https://nekos.best/)

pub extern crate reqwest;

use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// A response from the api
#[derive(serde::Deserialize, Debug, Clone, Hash)]
pub struct NekosBestResponse {
    /// The list of urls returned
    #[serde(deserialize_with = "serde_utils::string_or_seq_string")]
    pub url: Vec<String>,

    /// In the case of [`Category::Nekos`], also returns the source
    /// and the artist name and a link to their profile.
    /// It is empty in all other categories.
    #[serde(
        default,
        deserialize_with = "serde_utils::nekos_details_or_seq_nekos_details"
    )]
    pub details: Vec<NekosDetails>,
}

impl Index<usize> for NekosBestResponse {
    type Output = String;

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
    type Target = Vec<String>;

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
#[derive(Debug, Clone, Hash)]
pub struct NekosBestResponseSingle {
    /// The url
    pub url: String,
    /// The details, in case of [`Category::Nekos`]
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

impl TryFrom<NekosBestResponse> for NekosBestResponseSingle {
    type Error = NekosBestError;
    fn try_from(mut r: NekosBestResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            url: r.url.pop().ok_or(NekosBestError::NotFound)?,
            details: r.details.pop(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NekosBestError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("not found")]
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Baka,
    Cry,
    Cuddle,
    Dance,
    Feed,
    Hug,
    Kiss,
    Laugh,
    Nekos,
    Pat,
    Poke,
    Slap,
    Smile,
    Smug,
    Tickle,
    Wave,
}

impl Category {
    const fn to_url_path(self) -> &'static str {
        match self {
            Category::Baka => "baka",
            Category::Cry => "cry",
            Category::Cuddle => "cuddle",
            Category::Dance => "dance",
            Category::Feed => "feed",
            Category::Hug => "hug",
            Category::Kiss => "kiss",
            Category::Laugh => "laugh",
            Category::Nekos => "nekos",
            Category::Pat => "pat",
            Category::Poke => "poke",
            Category::Slap => "slap",
            Category::Smile => "smile",
            Category::Smug => "smug",
            Category::Tickle => "tickle",
            Category::Wave => "wave",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_url_path().fmt(f)
    }
}

pub const BASE_URL: &str = "https://nekos.best";

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_with_client(
    client: &reqwest::Client,
    category: impl Into<Category>,
) -> Result<NekosBestResponseSingle, NekosBestError> {
    let resp = get_with_client_amount(client, category, 1).await?;

    TryFrom::try_from(resp)
}

/// Gets `amount` images, with a supplied client.
/// Note that the server clamps the amount to the 1..=20 range
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_with_client_amount(
    client: &reqwest::Client,
    category: impl Into<Category>,
    amount: impl Into<Option<u8>>,
) -> Result<NekosBestResponse, NekosBestError> {
    let mut req = client.get(format!("{}/{}", BASE_URL, category.into()));
    let amount: Option<u8> = amount.into();
    if let Some(amount) = amount {
        req = req.query(&[("amount", amount)]);
    }

    let r: reqwest::Response = req.send().await?;

    let v = r.json::<NekosBestResponse>().await?;

    Ok(v)
}

/// Gets a single image, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get(category: impl Into<Category>) -> Result<NekosBestResponseSingle, NekosBestError> {
    let client = reqwest::Client::new();

    get_with_client(&client, category).await
}

/// Gets `amount` images, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_amount(
    category: impl Into<Category>,
    amount: impl Into<Option<u8>>,
) -> Result<NekosBestResponse, NekosBestError> {
    let client = reqwest::Client::new();

    get_with_client_amount(&client, category, amount).await
}

#[derive(serde::Deserialize)]
#[serde(try_from = "String")]
struct UrlEncodedString(String);

impl TryFrom<String> for UrlEncodedString {
    type Error = std::string::FromUtf8Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        urlencoding::decode(&s).map(|it| Self(it.into_owned()))
    }
}

#[derive(serde::Deserialize)]
struct NekosDetailsInternal {
    artist_href: UrlEncodedString,
    artist_name: UrlEncodedString,
    source_url: UrlEncodedString,
}

/// In the case of [`Category::Nekos`], the API
/// also returns the source url, the name and a
/// link to the artist that made it.
#[derive(serde::Deserialize, Debug, Clone, Hash)]
#[serde(from = "NekosDetailsInternal")]
pub struct NekosDetails {
    pub artist_href: String,
    pub artist_name: String,
    pub source_url: String,
}

impl From<NekosDetailsInternal> for NekosDetails {
    fn from(d: NekosDetailsInternal) -> Self {
        Self {
            artist_href: d.artist_href.0,
            artist_name: d.artist_name.0,
            source_url: d.source_url.0,
        }
    }
}

mod serde_utils {
    // serde helpers
    use std::fmt;

    use serde::{de, Deserialize, Deserializer};

    use super::NekosDetails;

    // from https://stackoverflow.com/a/43627388/12576629
    pub fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrVec;

        impl<'de> de::Visitor<'de> for StringOrVec {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or list of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![value.to_owned()])
            }

            fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
            }
        }

        deserializer.deserialize_any(StringOrVec)
    }

    pub fn nekos_details_or_seq_nekos_details<'de, D>(
        deserializer: D,
    ) -> Result<Vec<NekosDetails>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NekosDetailsOrVec;

        impl<'de> de::Visitor<'de> for NekosDetailsOrVec {
            type Value = Vec<NekosDetails>;

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

        deserializer.deserialize_option(NekosDetailsOrVec)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    async fn try_endpoint(
        client: &reqwest::Client,
        category: impl Into<Category>,
    ) -> Result<(), (NekosBestError, Category)> {
        let category = category.into();
        match get_with_client(client, category).await {
            Ok(_) => Ok(()),
            Err(e) => Err((e, category)),
        }
    }

    macro_rules! try_endpoints {
        ($client:expr, $try_endpoint_fn:ident, [$($(#[$at:meta])* $category:ident),* $(,)?]) => {
            $(try_endpoints!($client, $try_endpoint_fn, $(#[$at])* $category);)*
        };

        ($client:expr, $try_endpoint_fn:ident, $(#[$at:meta])* $category:ident) => {
            $try_endpoint_fn($client, $(#[$at])* {Category::$category}).await.unwrap(); // test will fail if any of them error
        }
    }

    #[tokio::test]
    async fn all_endpoints_work() {
        let client = reqwest::Client::new();
        try_endpoints!(
            &client,
            try_endpoint,
            [
                Baka, Cry, Cuddle, Dance, Feed, Hug, Kiss, Laugh, Nekos, Pat, Poke, Slap, Smile,
                Smug, Tickle, Wave,
            ]
        );
    }

    #[tokio::test]
    async fn no_new_endpoints() {
        let client = reqwest::Client::new();

        macro_rules! known_image_endpoints {
            ([$($(#[$at:meta])* $category:ident),* $(,)?]) => {
                [
                    $(
                        $(#[$at])* {known_image_endpoints!($category)},
                    )*
                ]
            };

            ($category:ident $(,)?) => {
                Category::$category.to_url_path()
            };
        }

        const KNOWN_ENDPOINTS: &[&str] = &known_image_endpoints!([
            Baka, Cry, Cuddle, Dance, Feed, Hug, Kiss, Laugh, Nekos, Pat, Poke, Slap, Smile, Smug,
            Tickle, Wave,
        ]);

        async fn get_endpoints(client: &reqwest::Client) -> HashMap<String, EndpointDesc> {
            client
                .get(format!("{}/endpoints", BASE_URL))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap()
        }

        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct EndpointDesc {
            min: String,
            max: String,
            format: String,
        }

        let endpoints = get_endpoints(&client).await;
        let list = endpoints.keys();

        let mut unknown_endpoints = vec![];
        for item in list {
            if !KNOWN_ENDPOINTS.contains(&item.as_str()) {
                unknown_endpoints.push(format!("{}/{}", BASE_URL, item));
            }
        }

        if !unknown_endpoints.is_empty() {
            panic!(
                "Looks like there are new endpoints, please add them: {:?}",
                unknown_endpoints
            );
        }
    }
}
