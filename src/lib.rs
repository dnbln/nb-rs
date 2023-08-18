//! API wrapper for [nekos.best](https://nekos.best/)

pub extern crate reqwest;

#[cfg(feature = "metrics")]
pub mod metrics;

pub mod category;
pub mod details;
pub mod response;
pub mod client;

pub use category::Category;
use url::ParseError;

pub use response::{NekosBestResponse, NekosBestResponseSingle};

#[derive(thiserror::Error, Debug)]
pub enum NekosBestError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("not found")]
    NotFound,

    #[error("decoding")]
    Decoding(#[from] serde_json::Error),

    #[error("decoding header values")]
    DecodingHeader(#[from] HeaderDeserializeUrlEncodedError),

    #[error("error parsing url")]
    UrlParseError(#[from] ParseError),

    #[error("rate limited")]
    RateLimited,
}

pub const API_VERSION: usize = 2;
pub const BASE_URL: &str = "https://nekos.best/api/v2";

#[cfg(feature = "strong-types")]
pub mod strong_types;

#[cfg(feature = "strong-types")]
pub use strong_types::*;

#[path = "impl.rs"]
mod implementation;

pub use implementation::*;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::client::{Client, ClientConfig};

    use super::*;

    async fn try_endpoint(
        client: &Client,
        category: impl Into<Category>,
    ) -> Result<(), (NekosBestError, Category)> {
        let category = category.into();
        match get_with_client(client, category).await {
            Ok(_) => Ok(()),
            Err(e) => Err((e, category)),
        }
    }

    #[tokio::test]
    async fn all_endpoints_work() {
        let client = Client::new(ClientConfig::default());
        for cat in Category::ALL_VARIANTS {
            try_endpoint(&client, *cat).await.unwrap(); // test will fail if any of them error
        }
    }

    #[tokio::test]
    async fn no_new_endpoints() {
        let client = reqwest::Client::new();

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
            if item.as_str().parse::<Category>().is_err() {
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
