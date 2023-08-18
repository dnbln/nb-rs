#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get_gif_details(
        "https://nekos.best/api/v2/pat/2c99b415-49fa-4e5c-8835-57a840e95730.gif",
    )
    .await?;
    println!("Anime name: {}", details.anime_name);
    Ok(())
}
