//! API wrapper for [nekos.best](https://nekos.best/)

pub extern crate reqwest;

#[cfg(feature = "local")]
pub mod local;

#[cfg(feature = "metrics")]
pub mod metrics;

pub mod category;
pub mod details;
pub mod response;

pub use category::Category;

use details::{GifDetails, NekosDetails};
use reqwest::IntoUrl;
pub use response::{NekosBestResponse, NekosBestResponseSingle};

#[derive(thiserror::Error, Debug)]
pub enum NekosBestError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("not found")]
    NotFound,

    #[error("decoding")]
    Decoding(#[from] serde_json::Error),
}

pub const API_VERSION: usize = 1;
pub const BASE_URL: &str = "https://nekos.best/api/v1";

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_with_client(
    client: &reqwest::Client,
    category: impl Into<Category>,
) -> Result<NekosBestResponseSingle, NekosBestError> {
    let r = client
        .get(format!("{}/{}", BASE_URL, category.into()))
        .send()
        .await?;

    let resp = r.error_for_status()?.json().await?;

    Ok(resp)
}

/// Gets `amount` images, with a supplied client.
/// Note that the server clamps the amount to the 1..=20 range
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_with_client_amount(
    client: &reqwest::Client,
    category: impl Into<Category>,
    amount: impl Into<u8>,
) -> Result<NekosBestResponse, NekosBestError> {
    let req = client
        .get(format!("{}/{}", BASE_URL, category.into()))
        .query(&[("amount", amount.into())]);

    let r: reqwest::Response = req.send().await?;

    let v = r.error_for_status()?.json::<NekosBestResponse>().await?;

    Ok(v)
}

/// Gets a single image, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get(category: impl Into<Category>) -> Result<NekosBestResponseSingle, NekosBestError> {
    get_with_client(&reqwest::Client::new(), category).await
}

/// Gets `amount` images, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_amount(
    category: impl Into<Category>,
    amount: impl Into<u8>,
) -> Result<NekosBestResponse, NekosBestError> {
    get_with_client_amount(&reqwest::Client::new(), category, amount).await
}

pub async fn get_with_client_nekos_details(client: &reqwest::Client, url: impl IntoUrl) -> Result<NekosDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let details = resp.headers().get("details").ok_or(NekosBestError::NotFound)?;
    let details_text = details.to_str().expect("Not ascii content in details");

    #[derive(serde::Deserialize)]
    #[serde(transparent)]
    struct UrlEncodedDetails {
        #[serde(deserialize_with = "crate::details::url_encoded_nekos_details_deserialize")]
        details: NekosDetails,
    }

    let d = serde_json::from_str::<UrlEncodedDetails>(&details_text)?;

    Ok(d.details)
}

pub async fn get_nekos_details(url: impl IntoUrl) -> Result<NekosDetails, NekosBestError> {
    get_with_client_nekos_details(&reqwest::Client::new(), url).await
}

pub async fn get_with_client_gif_details(client: &reqwest::Client, url: impl IntoUrl) -> Result<GifDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let details = resp.headers().get("details").ok_or(NekosBestError::NotFound)?;
    let details_text = details.to_str().expect("Not ascii content in details");

    #[derive(serde::Deserialize)]
    #[serde(transparent)]
    struct UrlEncodedDetails {
        #[serde(deserialize_with = "crate::details::url_encoded_gif_details_deserialize")]
        details: GifDetails,
    }

    let d = serde_json::from_str::<UrlEncodedDetails>(&details_text)?;

    Ok(d.details)
}

pub async fn get_gif_details(url: impl IntoUrl) -> Result<GifDetails, NekosBestError> {
    get_with_client_gif_details(&reqwest::Client::new(), url).await
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
                Smug, Tickle, Wave, Bite, Blush, Bored, Facepalm, Happy, Highfive, Pout, Shrug, Sleep, Stare,
                Think, ThumbsUp, Wink,
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
            Tickle, Wave, Bite, Blush, Bored, Facepalm, Happy, Highfive, Pout, Shrug, Sleep, Stare, Think,
            ThumbsUp, Wink,
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
