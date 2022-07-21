use nekosbest::{SearchQuery, SearchQueryKind};

#[tokio::main]
async fn main() {
    let r = nekosbest::search(
        SearchQuery::new("Senko", SearchQueryKind::Gif)
            .amount(2)
            .category(nekosbest::Category::Pat),
    )
    .await
    .unwrap();

    dbg!(&r.0);
}
