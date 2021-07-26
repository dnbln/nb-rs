#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get_amount(nekosbest::Category::Nekos, 0).await?;
    let img_urls = resp.url;
    println!("{:?}", img_urls);
    Ok(())
}
