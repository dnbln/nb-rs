use reqwest::IntoUrl;

use crate::{
    details::{GifDetails, NekosDetails},
    Category, NekosBestError, NekosBestResponse, NekosBestResponseSingle, BASE_URL,
};

type ReqwestClient = reqwest::Client;

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_with_client(
    client: &ReqwestClient,
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
pub async fn get(category: impl Into<Category>) -> Result<NekosBestResponseSingle, NekosBestError> {
    get_with_client(&ReqwestClient::new(), category).await
}

/// Gets `amount` images, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
pub async fn get_amount(
    category: impl Into<Category>,
    amount: impl Into<u8>,
) -> Result<NekosBestResponse, NekosBestError> {
    get_with_client_amount(&ReqwestClient::new(), category, amount).await
}

pub async fn get_with_client_nekos_details(
    client: &ReqwestClient,
    url: impl IntoUrl,
) -> Result<NekosDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let details = resp
        .headers()
        .get("details")
        .ok_or(NekosBestError::NotFound)?;
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
    get_with_client_nekos_details(&ReqwestClient::new(), url).await
}

pub async fn get_with_client_gif_details(
    client: &ReqwestClient,
    url: impl IntoUrl,
) -> Result<GifDetails, NekosBestError> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let details = resp
        .headers()
        .get("details")
        .ok_or(NekosBestError::NotFound)?;
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
    get_with_client_gif_details(&ReqwestClient::new(), url).await
}
