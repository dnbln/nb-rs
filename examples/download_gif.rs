#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Yawn).await?;
    let image = nekosbest::download::download(&resp).await?;
    tokio::task::spawn_blocking(move || image.save("yawn.gif")).await??;

    // or alternatively, if you just want to save it, without
    // loading the whole gif in-memory:
    nekosbest::download::download_to_file(&resp, "yawn2.gif").await?;

    Ok(())
}
