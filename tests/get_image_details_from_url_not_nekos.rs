#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Pat).await?;
    let img_url = &resp.url;
    assert_eq!(None, resp.details);

    let result = nekosbest::get_details(img_url).await;
    assert!(result.is_err());

    Ok(())
}
