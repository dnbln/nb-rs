//! API wrapper for [nekos.best](https://nekos.best/)

pub extern crate reqwest;

#[cfg(feature = "metrics")]
pub mod metrics;

pub mod category;
pub mod client;
pub mod details;
#[cfg(feature = "download")]
pub mod download;
pub mod response;

pub use category::Category;
use reqwest::header;
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

    #[cfg(feature = "download")]
    #[error("error decoding downloaded image")]
    ImageDecodeError(#[from] image::ImageError),

    #[error("missing content type")]
    MissingContentType,

    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("rate limited")]
    RateLimited,
}

pub const API_VERSION: usize = 2;
pub const BASE_URL: &str = "https://nekos.best/api/v2";
const API_CLIENT_AGENT: &str = concat!(
    "Cthulhu/",
    env!("CARGO_PKG_VERSION"),
    " (Unholy Terrors; Devouring Souls)"
);

#[cfg(feature = "strong-types")]
pub mod strong_types;

#[cfg(feature = "strong-types")]
pub use strong_types::*;

#[path = "impl.rs"]
mod implementation;

pub use implementation::*;

fn prepare_request(r: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    r.header(header::USER_AGENT, API_CLIENT_AGENT)
}

#[cfg(test)]
mod test {
    use crate::client::{Client, ClientConfig};
    use std::collections::HashMap;

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
            crate::prepare_request(client.get(format!("{BASE_URL}/endpoints")))
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
            format: String,
        }

        let endpoints = get_endpoints(&client).await;
        let list = endpoints.keys();

        let mut unknown_endpoints = vec![];
        for item in list {
            if item.as_str().parse::<Category>().is_err() {
                unknown_endpoints.push(format!("{BASE_URL}/{item}"));
            }
        }

        if !unknown_endpoints.is_empty() {
            panic!("Looks like there are new endpoints, please add them: {unknown_endpoints:?}");
        }
    }
}
