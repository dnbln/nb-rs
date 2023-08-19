#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = nekosbest::get(nekosbest::Category::Neko).await?;
    // let url: String = resp.url;
    let url = "https://nekos.best/api/v2/neko/1efcda2d-d0d3-4e96-9b40-86852374b4bc.png".to_owned();
    let image = nekosbest::download::download_from_url(&url).await?;
    tokio::task::spawn_blocking(move || image.save("neko.png")).await??;

    // or alternatively, if you just want to save it, without
    // loading the whole image in-memory:
    nekosbest::download::download_from_url_to_file(&url, "neko2.png").await?;

    Ok(())
}