#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Pat).await?;
    let img_url = resp.url;
    println!("{img_url}");
    Ok(())
}
