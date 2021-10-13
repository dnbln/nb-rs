use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::details::Details;

/// A response from the api
#[derive(serde::Deserialize, Debug, Clone, Hash)]
#[serde(transparent)]
pub struct NekosBestResponse {
    /// The list of urls returned, with artist and source details if
    /// using [`Category::Nekos`]
    pub url: Vec<NekosBestResponseSingle>,
}

impl Index<usize> for NekosBestResponse {
    type Output = NekosBestResponseSingle;

    fn index(&self, index: usize) -> &Self::Output {
        self.url.index(index)
    }
}

impl IndexMut<usize> for NekosBestResponse {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.url.index_mut(index)
    }
}

impl Deref for NekosBestResponse {
    type Target = Vec<NekosBestResponseSingle>;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl DerefMut for NekosBestResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.url
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
