#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get_gif_details("https://nekos.best/api/v2/pat/001.gif").await?;
    println!("Anime name: {}", details.anime_name);
    Ok(())
}
