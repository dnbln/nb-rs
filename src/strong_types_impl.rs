#[cfg(feature = "blocking")]
use nb_blocking_util::blocking;

use crate::client::{Client, ClientConfig};
use crate::{
    strong_types::{STCategory, STNekosBestResponse, STNekosBestResponseSingle},
    NekosBestError, STNekosBestSearchQuery, BASE_URL,
};

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client<C: STCategory>(
    client: &Client,
) -> Result<STNekosBestResponseSingle<C>, NekosBestError> {
    let r = client
        .client
        .get(format!("{BASE_URL}/{}", C::CATEGORY))
        .send()
        .await?;

    let mut resp = r
        .error_for_status()?
        .json::<STNekosBestResponse<C>>()
        .await?;
    let resp = resp.0.pop().ok_or(NekosBestError::NotFound)?;

    Ok(resp)
}

/// Gets `amount` images, with a supplied client.
/// Note that the server clamps the amount to the 1..=20 range
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client_amount<C: STCategory>(
    client: &Client,
    amount: impl Into<u8>,
) -> Result<STNekosBestResponse<C>, NekosBestError> {
    let req = client
        .client
        .get(format!("{BASE_URL}/{}", C::CATEGORY))
        .query(&[("amount", amount.into())]);

    let r = req.send().await?;

    let v = r.error_for_status()?.json().await?;

    Ok(v)
}

/// Gets a single image, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get<C: STCategory>() -> Result<STNekosBestResponseSingle<C>, NekosBestError> {
    get_with_client(&Client::new(ClientConfig::default())).await
}

/// Gets `amount` images, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_amount<C: STCategory>(
    amount: impl Into<u8>,
) -> Result<STNekosBestResponse<C>, NekosBestError> {
    get_with_client_amount(&Client::new(ClientConfig::default()), amount).await
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn search_with_client<C: STCategory>(
    client: &Client,
    query: STNekosBestSearchQuery<C>,
) -> Result<STNekosBestResponse<C>, NekosBestError> {
    #[cfg(not(feature = "blocking"))]
    client.handle_search_ratelimit().await?;

    let req = client.client.get(format!("{BASE_URL}/search"));

    let req = query.apply_to(req);

    let res = req.send().await?;

    #[cfg(not(feature = "blocking"))]
    client.update_search_ratelimit_data(res.headers()).await;

    Ok(res.error_for_status()?.json().await?)
}

#[deprecated(
    note = "Use `search_with_client` instead, and provide a client.",
    since = "0.17.0"
)]
#[cfg_attr(feature = "blocking", blocking)]
pub async fn search<C: STCategory>(
    query: STNekosBestSearchQuery<C>,
) -> Result<STNekosBestResponse<C>, NekosBestError> {
    search_with_client(&Client::new(ClientConfig::default()), query).await
}
