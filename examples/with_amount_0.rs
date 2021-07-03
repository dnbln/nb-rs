#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_urls: Vec<String> = nekosbest::get_amount(nekosbest::Category::Nekos, 0).await?;
    println!("{:?}", img_urls);
    Ok(())
}
