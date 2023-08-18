use reqwest::header::HeaderMap;
use reqwest::IntoUrl;
use serde::Serializer;
use std::string::FromUtf8Error;

use crate::{
    details::{GifDetails, ImageDetails},
    Category, NekosBestError, NekosBestResponse, NekosBestResponseSingle, BASE_URL,
};

#[cfg(feature = "blocking")]
use nb_blocking_util::blocking;

#[cfg(feature = "strong-types")]
#[path = "strong_types_impl.rs"]
mod strong_types_impl;

use crate::client::{Client, ClientConfig, ReqBuilder, ReqwestResponse};
#[cfg(feature = "strong-types")]
pub use strong_types_impl::{
    get as st_get, get_amount as st_get_amount, get_with_client as st_get_with_client,
    get_with_client_amount as st_get_with_client_amount, search as st_search,
    search_with_client as st_search_with_client,
};

#[cfg_attr(feature = "blocking", blocking)]
async fn parse_from_response(
    response: ReqwestResponse,
) -> Result<NekosBestResponse, NekosBestError> {
    #[cfg(not(nekosbest_dbgjson))]
    let v = response
        .error_for_status()?
        .json::<NekosBestResponse>()
        .await?;

    #[cfg(nekosbest_dbgjson)]
    let v = {
        let json = response.error_for_status()?.text().await?;
        dbg!(&json);
        serde_json::from_str::<NekosBestResponse>(&json)?
    };

    Ok(v)
}

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client(
    client: &Client,
    category: impl Into<Category>,
) -> Result<NekosBestResponseSingle, NekosBestError> {
    let r = client
        .client
        .get(format!("{BASE_URL}/{}", category.into()))
        .send()
        .await?;

    let mut resp = parse_from_response(r).await?;
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
    client: &Client,
    category: impl Into<Category>,
    amount: impl Into<u8>,
) -> Result<NekosBestResponse, NekosBestError> {
    let req = client
        .client
        .get(format!("{BASE_URL}/{}", category.into()))
        .query(&[("amount", amount.into())]);

    let r = req.send().await?;

    let v = parse_from_response(r).await?;

    Ok(v)
}

/// Gets a single image, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get(category: impl Into<Category>) -> Result<NekosBestResponseSingle, NekosBestError> {
    get_with_client(&Client::new(ClientConfig::default()), category).await
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
    get_with_client_amount(&Client::new(ClientConfig::default()), category, amount).await
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
pub async fn get_with_client_image_details(
    client: &Client,
    url: impl IntoUrl,
) -> Result<ImageDetails, NekosBestError> {
    let resp = client.client.get(url).send().await?.error_for_status()?;
    let headers = resp.headers();

    let details = ImageDetails {
        artist_name: header_deserialize_urlencoded(headers, "artist_name")?,
        artist_href: header_deserialize_urlencoded(headers, "artist_href")?.parse()?,
        source_url: header_deserialize_urlencoded(headers, "source_url")?.parse()?,
    };

    Ok(details)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_image_details(url: impl IntoUrl) -> Result<ImageDetails, NekosBestError> {
    get_with_client_image_details(&Client::new(ClientConfig::default()), url).await
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client_gif_details(
    client: &Client,
    url: impl IntoUrl,
) -> Result<GifDetails, NekosBestError> {
    let resp = client.client.get(url).send().await?.error_for_status()?;
    let headers = resp.headers();

    let details = GifDetails {
        anime_name: header_deserialize_urlencoded(headers, "anime_name")?,
    };

    Ok(details)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_gif_details(url: impl IntoUrl) -> Result<GifDetails, NekosBestError> {
    get_with_client_gif_details(&Client::new(ClientConfig::default()), url).await
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

    fn apply_to(&self, r: ReqBuilder) -> ReqBuilder {
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
    client: &Client,
    query: SearchQuery,
) -> Result<NekosBestResponse, NekosBestError> {
    let req = client.client.get(format!("{BASE_URL}/search"));

    #[cfg(not(feature = "blocking"))]
    client.handle_search_ratelimit().await?;

    let req = query.apply_to(req);

    let res = req.send().await?;

    #[cfg(not(feature = "blocking"))]
    client.update_search_ratelimit_data(res.headers()).await;

    Ok(parse_from_response(res).await?)
}

#[deprecated(
    note = "Use `search_with_client` instead, and provide a client.",
    since = "0.17.0"
)]
#[cfg_attr(feature = "blocking", blocking)]
pub async fn search(query: SearchQuery) -> Result<NekosBestResponse, NekosBestError> {
    search_with_client(&Client::new(ClientConfig::default()), query).await
}
