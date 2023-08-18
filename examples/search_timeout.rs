use nekosbest::client::ClientConfig;
use nekosbest::{SearchQuery, SearchQueryKind};

#[tokio::main]
async fn main() {
    let client = nekosbest::client::Client::new(ClientConfig {
        search_ratelimit_behavior: nekosbest::client::SearchRatelimitBehavior::Error,
    });

    for i in 0..100 {
        let r = nekosbest::search_with_client(
            &client,
            SearchQuery::new("Senko", SearchQueryKind::Gif)
                .amount(2)
                .category(nekosbest::Category::Pat),
        )
        .await;

        match r {
            Ok(r) => {
                dbg!(&r.0);
            }
            Err(nekosbest::NekosBestError::RateLimited) => {
                println!("Rate limited on iteration {i}");
                return;
            }
            Err(e) => {
                panic!("Error: {e:?}");
            }
        }
    }
}
