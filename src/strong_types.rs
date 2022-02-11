use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use serde::Deserialize;

use crate::{
    details::{GifDetails, NekoDetails},
    Category,
};

pub trait STCategory: Sized {
    const CATEGORY: Category;
    type Details: for<'de> Deserialize<'de> + Debug + Clone + 'static;
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct STNekosBestResponseSingle<C: STCategory> {
    pub url: String,
    #[serde(flatten)]
    pub details: C::Details,
}

impl<C: STCategory> STNekosBestResponseSingle<C> {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn details(&self) -> &C::Details {
        &self.details
    }
}

macro_rules! gif_endpoints {
    ([$($name:ident),* $(,)?]) => {
        $(
            gif_endpoints!($name);
        )*
    };
    ($name:ident) => {
        #[derive(Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;

        impl STCategory for $name {
            const CATEGORY: Category = Category::$name;
            type Details = GifDetails;
        }
    };
}

gif_endpoints!([
    Baka, Cry, Cuddle, Dance, Feed, Hug, Kiss, Laugh, Pat, Poke, Slap, Smile, Smug, Tickle, Wave,
    Bite, Blush, Bored, Facepalm, Happy, Highfive, Pout, Shrug, Sleep, Stare, Think, ThumbsUp,
    Wink,
]);

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Neko;

impl STCategory for Neko {
    const CATEGORY: Category = Category::Neko;
    type Details = NekoDetails;
}

#[deprecated(since = "0.11.0", note = "Use `Neko` instead")]
pub type Nekos = Neko;

#[derive(Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "C: STCategory"))]
struct STNekosBestResponseV2<C>
where
    C: STCategory,
{
    results: Vec<STNekosBestResponseSingle<C>>,
}

/// A response from the api
#[derive(Debug, Clone)]
pub struct STNekosBestResponse<C>(pub Vec<STNekosBestResponseSingle<C>>)
where
    C: STCategory;

impl<'de, C: STCategory> Deserialize<'de> for STNekosBestResponse<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        STNekosBestResponseV2::<C>::deserialize(deserializer).map(Self::from)
    }
}

impl<C> From<STNekosBestResponseV2<C>> for STNekosBestResponse<C>
where
    C: STCategory,
{
    fn from(v: STNekosBestResponseV2<C>) -> Self {
        Self(v.results)
    }
}

impl<C: STCategory> Index<usize> for STNekosBestResponse<C> {
    type Output = STNekosBestResponseSingle<C>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: STCategory> IndexMut<usize> for STNekosBestResponse<C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: STCategory> Deref for STNekosBestResponse<C> {
    type Target = Vec<STNekosBestResponseSingle<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: STCategory> DerefMut for STNekosBestResponse<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
