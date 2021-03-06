use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::details::Details;

#[derive(serde::Deserialize, Debug, Clone, Hash)]
struct NekosBestResponseV2 {
    results: Vec<NekosBestResponseSingle>,
}

/// A response from the api
#[derive(serde::Deserialize, Debug, Clone, Hash)]
#[serde(from = "NekosBestResponseV2")]
pub struct NekosBestResponse(pub Vec<NekosBestResponseSingle>);

impl From<NekosBestResponseV2> for NekosBestResponse {
    fn from(r: NekosBestResponseV2) -> Self {
        NekosBestResponse(r.results)
    }
}

impl Index<usize> for NekosBestResponse {
    type Output = NekosBestResponseSingle;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for NekosBestResponse {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Deref for NekosBestResponse {
    type Target = Vec<NekosBestResponseSingle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NekosBestResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A response from the api, in the case of requesting a single
/// url with [`get`] or [`get_with_client`]
#[derive(Debug, Clone, Hash, serde::Deserialize)]
pub struct NekosBestResponseSingle {
    /// The url
    pub url: String,
    /// The details
    #[serde(flatten)]
    pub details: Details,
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
