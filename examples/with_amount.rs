#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get_amount(nekosbest::Category::Nekos, 3).await?;
    let img_urls = resp.0;
    println!("{:?}", img_urls);
    Ok(())
}
