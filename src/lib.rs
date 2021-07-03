#[derive(thiserror::Error, Debug)]
pub enum NekosBestError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("not found")]
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Baka,
    Cry,
    Cuddle,
    Dance,
    Feed,
    Hug,
    Kiss,
    Laugh,
    Nekos,
    Pat,
    Poke,
    Slap,
    Smile,
    Smug,
    Tickle,
    Wave,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Baka => "baka",
            Category::Cry => "cry",
            Category::Cuddle => "cuddle",
            Category::Dance => "dance",
            Category::Feed => "feed",
            Category::Hug => "hug",
            Category::Kiss => "kiss",
            Category::Laugh => "laugh",
            Category::Nekos => "nekos",
            Category::Pat => "pat",
            Category::Poke => "poke",
            Category::Slap => "slap",
            Category::Smile => "smile",
            Category::Smug => "smug",
            Category::Tickle => "tickle",
            Category::Wave => "wave",
        }
        .fmt(f)
    }
}

pub const BASE_URL: &str = "https://nekos.best";
pub const AMOUNT_LIMIT: u8 = 20;

pub async fn get_with_client(
    client: &reqwest::Client,
    category: impl Into<Category>,
) -> Result<String, NekosBestError> {
    let mut resp = get_with_client_amount(client, category, 1).await?;

    Ok(resp.pop().ok_or(NekosBestError::NotFound)?)
}

pub async fn get_with_client_amount(
    client: &reqwest::Client,
    category: impl Into<Category>,
    amount: impl Into<Option<u8>>,
) -> Result<Vec<String>, NekosBestError> {
    use std::{fmt, marker::PhantomData};

    use serde::{de, Deserialize, Deserializer};

    let mut req = client.get(format!("{}/{}", BASE_URL, category.into()));
    let amount: Option<u8> = amount.into();
    if let Some(amount) = amount {
        req = req.query(&[("amount", amount.clamp(1, AMOUNT_LIMIT))]);
    }

    let r: reqwest::Response = req.send().await?;

    #[derive(Deserialize)]
    struct NekosBestResponse {
        #[serde(deserialize_with = "string_or_seq_string")]
        url: Vec<String>,
    }

    // from https://stackoverflow.com/a/43627388/12576629
    fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrVec(PhantomData<Vec<String>>);

        impl<'de> de::Visitor<'de> for StringOrVec {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or list of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![value.to_owned()])
            }

            fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
            }
        }

        deserializer.deserialize_any(StringOrVec(PhantomData))
    }

    let v = r.json::<NekosBestResponse>().await?;

    Ok(v.url)
}

pub async fn get(category: impl Into<Category>) -> Result<String, NekosBestError> {
    let client = reqwest::Client::new();

    get_with_client(&client, category).await
}

pub async fn get_amount(
    category: impl Into<Category>,
    amount: impl Into<Option<u8>>,
) -> Result<Vec<String>, NekosBestError> {
    let client = reqwest::Client::new();

    get_with_client_amount(&client, category, amount).await
}
