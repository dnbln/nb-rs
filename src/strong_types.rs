use reqwest::RequestBuilder;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use serde::Deserialize;

use crate::{
    details::{GifDetails, ImageDetails},
    Category,
};

pub trait STCategory: Sized {
    const CATEGORY: Category;
    type Details: for<'de> Deserialize<'de> + Debug + Clone + 'static;
    type SearchQueryType: STNekosBestSearchQueryType;
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
            type SearchQueryType = STNekosBestSearchQueryGifType;
        }
    };
}

gif_endpoints!([
    Baka, Bite, Blush, Bored, Cry, Cuddle, Dance, Facepalm, Feed, Handhold, Happy, Highfive, Hug,
    Kick, Kiss, Laugh, Pat, Poke, Pout, Punch, Shoot, Shrug, Slap, Sleep, Smile, Smug, Stare,
    Think, ThumbsUp, Tickle, Wave, Wink, Yeet
]);

macro_rules! image_endpoints {
    ([$($name:ident),* $(,)?]) => {
        $(
            image_endpoints!($name);
        )*
    };
    ($name:ident) => {
        #[derive(Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;

        impl STCategory for $name {
            const CATEGORY: Category = Category::$name;
            type Details = ImageDetails;
            type SearchQueryType = STNekosBestSearchQueryImageType;
        }
    };
}

image_endpoints!([Husbando, Kitsune, Neko, Waifu]);

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
pub struct STNekosBestResponse<C: STCategory>(pub Vec<STNekosBestResponseSingle<C>>);

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

#[derive(serde::Serialize)]
pub struct STNekosBestSearchQuery<C: STCategory> {
    pub(crate) query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) amount: Option<usize>,

    #[serde(skip)]
    _phantom: std::marker::PhantomData<C>,
}

impl<C: STCategory> STNekosBestSearchQuery<C> {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            amount: None,

            _phantom: std::marker::PhantomData,
        }
    }

    pub fn amount(mut self, amount: usize) -> Self {
        self.amount = Some(amount);
        self
    }

    pub(crate) fn apply_to(&self, r: RequestBuilder) -> RequestBuilder {
        r.query(self)
            .query(&[("type", C::SearchQueryType::TYPE)])
            .query(&[("category", C::CATEGORY.to_url_path())])
    }
}

pub trait STNekosBestSearchQueryType {
    const TYPE: i32;
}

pub struct STNekosBestSearchQueryImageType;

impl STNekosBestSearchQueryType for STNekosBestSearchQueryImageType {
    const TYPE: i32 = 1;
}

pub struct STNekosBestSearchQueryGifType;

impl STNekosBestSearchQueryType for STNekosBestSearchQueryGifType {
    const TYPE: i32 = 2;
}
