#[tokio::test]
async fn try_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let _ = nekosbest::metrics::get_metrics().await.unwrap();

    Ok(())
}