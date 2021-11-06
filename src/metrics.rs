use std::{collections::HashMap, convert::TryFrom};

use crate::{category::NoSuchVariant, Category, NekosBestError, BASE_URL};

#[derive(serde::Deserialize)]
struct MetricsInternal {
    per_id: HashMap<String, String>,
    per_random: HashMap<String, String>,
    #[serde(with = "humantime_serde")]
    reset_in: std::time::Duration,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseMetricsError {
    #[error("parse int")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("no such variant")]
    NoSuchVariant(#[from] NoSuchVariant),
}

impl TryFrom<MetricsInternal> for Metrics {
    type Error = ParseMetricsError;

    fn try_from(
        MetricsInternal {
            per_id,
            per_random,
            reset_in,
        }: MetricsInternal,
    ) -> Result<Self, Self::Error> {
        Ok(Metrics {
            per_id: per_id
                .into_iter()
                .map(
                    |(category, count)| -> Result<(Category, usize), ParseMetricsError> {
                        Ok((category.parse()?, count.parse()?))
                    },
                )
                .collect::<Result<_, ParseMetricsError>>()?,
            per_random: per_random
                .into_iter()
                .map(
                    |(category, count)| -> Result<(Category, usize), ParseMetricsError> {
                        Ok((category.parse()?, count.parse()?))
                    },
                )
                .collect::<Result<_, ParseMetricsError>>()?,
            reset_in,
        })
    }
}

#[derive(serde::Deserialize)]
#[serde(try_from = "MetricsInternal")]
pub struct Metrics {
    pub per_id: HashMap<Category, usize>,
    pub per_random: HashMap<Category, usize>,
    pub reset_in: std::time::Duration,
}

pub async fn get_metrics_with_client(client: &reqwest::Client) -> Result<Metrics, NekosBestError> {
    let resp = client.get(format!("{}/metrics", BASE_URL)).send().await?;

    let metrics = resp.json::<Metrics>().await?;

    Ok(metrics)
}

pub async fn get_metrics() -> Result<Metrics, NekosBestError> {
    get_metrics_with_client(&reqwest::Client::new()).await
}
