#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Nekos).await?;
    let img_url = &resp.url;

    let s = nekosbest::get_details(img_url).await?;
    assert_eq!(Some(&s), resp.details.as_ref());

    Ok(())
}
