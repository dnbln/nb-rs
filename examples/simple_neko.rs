#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Nekos).await?;
    let img_url = &resp.url;
    println!("{}", img_url);
    Ok(())
}
