#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get(nekosbest::Category::Pat).await?.details.try_into_gif().unwrap();
    println!("Anime name: {}", details.anime_name);
    Ok(())
}
