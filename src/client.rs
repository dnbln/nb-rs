use std::ops::Add;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[cfg(not(feature = "blocking"))]
pub(crate) type ReqwestClient = reqwest::Client;
#[cfg(feature = "blocking")]
pub(crate) type ReqwestClient = reqwest::blocking::Client;

#[cfg(not(feature = "blocking"))]
pub(crate) type ReqBuilder = reqwest::RequestBuilder;
#[cfg(feature = "blocking")]
pub(crate) type ReqBuilder = reqwest::blocking::RequestBuilder;

struct SearchRatelimitData {
    remaining: u32,
    resets_at: Instant,
}

#[derive(Copy, Clone, Default)]
pub enum SearchRatelimitBehavior {
    #[default]
    Sleep,
    Error,
}

#[derive(Default, Copy, Clone)]
pub struct ClientConfig {
    pub search_ratelimit_behavior: SearchRatelimitBehavior,
}

pub struct Client {
    pub(crate) client: ReqwestClient,
    client_config: ClientConfig,
    search_ratelimit_data: Arc<Mutex<Option<SearchRatelimitData>>>,
}

impl Client {
    pub fn new(client_config: ClientConfig) -> Self {
        Self::new_with_reqwest_client(ReqwestClient::new(), client_config)
    }

    pub fn new_with_reqwest_client(client: ReqwestClient, client_config: ClientConfig) -> Self {
        Self {
            client,
            client_config,
            search_ratelimit_data: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) async fn handle_search_ratelimit(&self) -> Result<(), crate::NekosBestError> {
        let lock = self.search_ratelimit_data.lock().await;
        if let Some(search_ratelimit_data) = &*lock {
            if search_ratelimit_data.remaining == 0 {
                match self.client_config.search_ratelimit_behavior {
                    SearchRatelimitBehavior::Sleep => {
                        tokio::time::sleep_until(search_ratelimit_data.resets_at).await;
                    }
                    SearchRatelimitBehavior::Error => {
                        return Err(crate::NekosBestError::RateLimited);
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) async fn update_search_ratelimit_data(&self, headers: &reqwest::header::HeaderMap) {
        let Some(remaining) = headers.get("X-Rate-Limit-Remaining")
            else { return; };
        let Some(reset) = headers.get("X-Rate-Limit-Reset")
            else { return; };
        let Ok(remaining) = remaining.to_str() else { return; };
        let Ok(remaining) = remaining.parse::<u32>() else { return; };
        let Ok(reset) = reset.to_str() else { return; };
        dbg!(reset);

        let Ok(reset) = reset.parse::<chrono::DateTime::<chrono::Utc>>()
            else { return; };

        let diff = reset - chrono::Utc::now();

        if diff < chrono::Duration::zero() {
            *self.search_ratelimit_data.lock().await = None;
            return;
        }

        *self.search_ratelimit_data.lock().await = Some(SearchRatelimitData {
            remaining,
            resets_at: Instant::from_std(std::time::Instant::now().add(diff.to_std().unwrap())),
        });
    }
}
