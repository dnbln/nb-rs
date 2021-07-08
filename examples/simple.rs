#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url: String = nekosbest::get(nekosbest::Category::Pat).await?;
    println!("{}", img_url);
    Ok(())
}
