use reqwest::header::HeaderMap;
use reqwest::IntoUrl;
use std::string::FromUtf8Error;

use crate::{
    details::{GifDetails, NekoDetails},
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
    get_with_client_amount as st_get_with_client_amount,
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
        .get(format!("{}/{}", BASE_URL, category.into()))
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
        .get(format!("{}/{}", BASE_URL, category.into()))
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
) -> Result<NekoDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let headers = resp.headers();

    let details = NekoDetails {
        artist_name: header_deserialize_urlencoded(headers, "artist_name")?,
        artist_href: header_deserialize_urlencoded(headers, "artist_href")?.parse()?,
        source_url: header_deserialize_urlencoded(headers, "source_url")?.parse()?,
    };

    Ok(details)
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_neko_details(url: impl IntoUrl) -> Result<NekoDetails, NekosBestError> {
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
