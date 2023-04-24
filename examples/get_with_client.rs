#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = nekosbest::client::Client::new(nekosbest::client::ClientConfig::default());
    let details = nekosbest::get_with_client(&client, nekosbest::Category::Neko)
        .await?
        .details
        .try_into_image()
        .unwrap();
    println!("Source: {}", details.source_url);
    println!("Artist: {}", details.artist_name);
    println!("Artist link: {}", details.artist_href);
    Ok(())
}
