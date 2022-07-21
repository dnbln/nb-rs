use reqwest::header::HeaderMap;
use reqwest::{IntoUrl, RequestBuilder};
use serde::Serializer;
use std::string::FromUtf8Error;

use crate::{
    details::{GifDetails, ImageDetails},
    Category, NekosBestError, NekosBestResponse, NekosBestResponseSingle, BASE_URL,
};

#[cfg(feature = "blocking")]
use nb_blocking_util::blocking;

#[cfg(not(feature = "blocking"))]
type ReqwestClient = reqwest::Client;
#[cfg(feature = "blocking")]
type ReqwestClient = reqwest::blocking::Client;

#[cfg(feature = "strong-types")]
#[path = "strong_types_impl.rs"]
mod strong_types_impl;

#[cfg(feature = "strong-types")]
pub use strong_types_impl::{
    get as st_get, get_amount as st_get_amount, get_with_client as st_get_with_client,
    get_with_client_amount as st_get_with_client_amount, search as st_search,
    search_with_client as st_search_with_client,
};

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client(
    client: &ReqwestClient,
    category: impl Into<Category>,
) -> Result<NekosBestResponseSingle, NekosBestError> {
    let r = client
        .get(format!("{BASE_URL}/{}", category.into()))
        .send()
        .await?;

    let mut resp = r.error_for_status()?.json::<NekosBestResponse>().await?;
    let resp = resp.0.pop().ok_or(NekosBestError::NotFound)?;

    Ok(resp)
}

/// Gets `amount` images, with a supplied client.
/// Note that the server clamps the amount to the 1..=20 range
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client_amount(
    client: &ReqwestClient,
    category: impl Into<Category>,
    amount: impl Into<u8>,
) -> Result<NekosBestResponse, NekosBestError> {
    let req = client
        .get(format!("{BASE_URL}/{}", category.into()))
        .query(&[("amount", amount.into())]);

    let r = req.send().await?;

    let v = r.error_for_status()?.json::<NekosBestResponse>().await?;

    Ok(v)
}

/// Gets a single image, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get(category: impl Into<Category>) -> Result<NekosBestResponseSingle, NekosBestError> {
    get_with_client(&ReqwestClient::new(), category).await
}

/// Gets `amount` images, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_amount(
    category: impl Into<Category>,
    amount: impl Into<u8>,
) -> Result<NekosBestResponse, NekosBestError> {
    get_with_client_amount(&ReqwestClient::new(), category, amount).await
}

#[derive(Debug, thiserror::Error)]
pub enum HeaderDeserializeUrlEncodedError {
    #[error("Missing header")]
    MissingHeader,
    #[error("Not ASCII header")]
    NotAsciiHeader(#[from] reqwest::header::ToStrError),
    #[error("UTF8 error")]
    Utf8(#[from] FromUtf8Error),
}

fn header_deserialize_urlencoded(
    headers: &HeaderMap,
    name: &str,
) -> Result<String, HeaderDeserializeUrlEncodedError> {
    let s = headers
        .get(name)
        .ok_or(HeaderDeserializeUrlEncodedError::MissingHeader)?
        .to_str()?;

    let s = urlencoding::decode(s)?.replace("+", " ");

    Ok(s)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client_neko_details(
    client: &ReqwestClient,
    url: impl IntoUrl,
) -> Result<ImageDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let headers = resp.headers();

    let details = ImageDetails {
        artist_name: header_deserialize_urlencoded(headers, "artist_name")?,
        artist_href: header_deserialize_urlencoded(headers, "artist_href")?.parse()?,
        source_url: header_deserialize_urlencoded(headers, "source_url")?.parse()?,
    };

    Ok(details)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_neko_details(url: impl IntoUrl) -> Result<ImageDetails, NekosBestError> {
    get_with_client_neko_details(&ReqwestClient::new(), url).await
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client_gif_details(
    client: &ReqwestClient,
    url: impl IntoUrl,
) -> Result<GifDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let headers = resp.headers();

    let details = GifDetails {
        anime_name: header_deserialize_urlencoded(headers, "anime_name")?,
    };

    Ok(details)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_gif_details(url: impl IntoUrl) -> Result<GifDetails, NekosBestError> {
    get_with_client_gif_details(&ReqwestClient::new(), url).await
}

#[derive(serde::Serialize)]
pub struct SearchQuery {
    query: String,
    #[serde(rename = "type")]
    kind: SearchQueryKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<usize>,
}

impl SearchQuery {
    pub fn new(query: impl Into<String>, kind: impl Into<SearchQueryKind>) -> Self {
        Self {
            query: query.into(),
            kind: kind.into(),
            category: None,
            amount: None,
        }
    }

    pub fn category(mut self, category: impl Into<Category>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn amount(mut self, amount: usize) -> Self {
        self.amount = Some(amount);
        self
    }

    fn apply_to(&self, r: RequestBuilder) -> RequestBuilder {
        r.query(self)
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SearchQueryKind {
    Image = 1,
    Gif = 2,
}

impl serde::Serialize for SearchQueryKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn search_with_client(
    client: &ReqwestClient,
    query: SearchQuery,
) -> Result<NekosBestResponse, NekosBestError> {
    let req = client.get(format!("{BASE_URL}/search"));

    let req = query.apply_to(req);

    Ok(req
        .send()
        .await?
        .error_for_status()?
        .json::<NekosBestResponse>()
        .await?)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn search(query: SearchQuery) -> Result<NekosBestResponse, NekosBestError> {
    search_with_client(&ReqwestClient::new(), query).await
}
