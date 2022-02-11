#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Neko).await?;
    let img_url = &resp.url;
    println!("{}", img_url);
    Ok(())
}
