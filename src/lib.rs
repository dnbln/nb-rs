use reqwest::Client;
use serde::Deserialize;

#[macro_use]
extern crate thiserror;

#[derive(Error, Debug)]
pub enum NekosBestError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
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

pub async fn get_with_client(
    client: &Client,
    category: Category,
) -> Result<String, NekosBestError> {
    let r: reqwest::Response = client
        .get(format!("{}/{}", BASE_URL, category))
        .send()
        .await?;

    #[derive(Deserialize)]
    struct NekosBestResponse {
        url: String,
    }

    let v = r.json::<NekosBestResponse>().await?;

    Ok(v.url)
}

pub async fn get_with_client_amount(
    client: &Client,
    category: Category,
    amount: u8,
) -> Result<Vec<String>, NekosBestError> {
    let r: reqwest::Response = client
        .get(format!("{}/{}?amount={}", BASE_URL, category, amount))
        .send()
        .await?;

    #[derive(Deserialize)]
    struct NekosBestResponse {
        url: Vec<String>,
    }

    let v = r.json::<NekosBestResponse>().await?;

    Ok(v.url)
}

pub async fn get(category: Category) -> Result<String, NekosBestError> {
    let client = Client::new();

    get_with_client(&client, category).await
}

pub async fn get_amount(category: Category, amount: u8) -> Result<Vec<String>, NekosBestError> {
    let client = Client::new();

    get_with_client_amount(&client, category, amount).await
}
