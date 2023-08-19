#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Neko).await?;

    let image = nekosbest::download::download(&resp).await?;
    tokio::task::spawn_blocking(move || image.save("neko.png")).await??;

    // or alternatively, if you just want to save it, without
    // loading the whole image in-memory:
    nekosbest::download::download_to_file(&resp, "neko2.png").await?;
    Ok(())
}
