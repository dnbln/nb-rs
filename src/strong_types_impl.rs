#[cfg(feature = "blocking")]
use nb_blocking_util::blocking;

use crate::{
    strong_types::{STCategory, STNekosBestResponse, STNekosBestResponseSingle},
    NekosBestError, BASE_URL,
};

use super::ReqwestClient;

/// Gets a single image, with a supplied client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client<C: STCategory>(
    client: &ReqwestClient,
) -> Result<STNekosBestResponseSingle<C>, NekosBestError> {
    let r = client
        .get(format!("{}/{}", BASE_URL, C::CATEGORY))
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
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_with_client_amount<C: STCategory>(
    client: &ReqwestClient,
    amount: impl Into<u8>,
) -> Result<STNekosBestResponse<C>, NekosBestError> {
    let req = client
        .get(format!("{}/{}", BASE_URL, C::CATEGORY))
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
    get_with_client(&ReqwestClient::new()).await
}

/// Gets `amount` images, with the default client.
///
/// # Errors
/// Any errors that can happen, refer to [`NekosBestError`].
#[cfg_attr(feature = "blocking", blocking)]
pub async fn get_amount<C: STCategory>(
    amount: impl Into<u8>,
) -> Result<STNekosBestResponse<C>, NekosBestError> {
    get_with_client_amount(&ReqwestClient::new(), amount).await
}
