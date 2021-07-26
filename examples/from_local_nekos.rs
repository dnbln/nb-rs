#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url = nekosbest::local::Nekos.get();
    println!("{}", img_url);
    Ok(())
}
