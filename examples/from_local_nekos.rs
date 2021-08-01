#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url = nekosbest::local::Nekos.get(); // requires the "local" feature
    println!("{}", img_url);
    Ok(())
}
