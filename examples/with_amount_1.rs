#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get_amount(nekosbest::Category::Neko, 1).await?;
    let img_urls = resp.0;
    println!("{img_urls:?}");
    Ok(())
}
