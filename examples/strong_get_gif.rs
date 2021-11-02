#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::st_get::<nekosbest::Pat>().await?;
    let img_url = resp.url();
    println!("{}", img_url);
    let details = resp.details();
    println!("Anime name: {}", details.anime_name);
    Ok(())
}
