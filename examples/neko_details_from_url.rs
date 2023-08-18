#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get_image_details(
        "https://nekos.best/api/v2/neko/1ee4ad62-d0fd-4956-adeb-05b6917f0a31.png",
    )
    .await?;
    println!("Source: {}", details.source_url);
    println!("Artist: {}", details.artist_name);
    println!("Artist link: {}", details.artist_href);
    Ok(())
}
