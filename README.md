# nb-rs

Rust API wrapper for [nekos.best](https://nekos.best/).


## Example
```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url = nekosbest::get(nekosbest::Category::Nekos).await?;
    println!("{}", img_url);
    Ok(())
}
```


