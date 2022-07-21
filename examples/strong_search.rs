use nekosbest::STNekosBestSearchQuery;

#[tokio::main]
async fn main() {
    let r = nekosbest::st_search(STNekosBestSearchQuery::<nekosbest::Pat>::new("Senko").amount(2))
        .await
        .unwrap();

    dbg!(&r.0);
}
