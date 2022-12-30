use std::str::FromStr;
use std::{collections::HashMap, convert::TryFrom};

use crate::{category::NoSuchVariant, Category, NekosBestError, BASE_URL};

#[derive(serde::Deserialize)]
struct MetricsInternal {
    per_id: HashMap<String, String>,
    per_random: HashMap<String, String>,
    total: String,
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
            total,
            reset_in,
        }: MetricsInternal,
    ) -> Result<Self, Self::Error> {
        Ok(Metrics {
            per_id: per_id
                .into_iter()
                .map(
                    |(category, count)| -> Result<(CategoryOrTotal, usize), ParseMetricsError> {
                        Ok((category.parse()?, count.parse()?))
                    },
                )
                .collect::<Result<_, ParseMetricsError>>()?,
            per_random: per_random
                .into_iter()
                .map(
                    |(category, count)| -> Result<(CategoryOrTotal, usize), ParseMetricsError> {
                        Ok((category.parse()?, count.parse()?))
                    },
                )
                .collect::<Result<_, ParseMetricsError>>()?,
            total: total.parse()?,
            reset_in,
        })
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CategoryOrTotal {
    Category(Category),
    Total,
}

impl FromStr for CategoryOrTotal {
    type Err = NoSuchVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "total" => Ok(CategoryOrTotal::Total),
            _ => Ok(CategoryOrTotal::Category(s.parse()?)),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(try_from = "MetricsInternal")]
pub struct Metrics {
    pub per_id: HashMap<CategoryOrTotal, usize>,
    pub per_random: HashMap<CategoryOrTotal, usize>,
    pub total: usize,
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
