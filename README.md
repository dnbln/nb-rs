# nb-rs

Rust API wrapper for [nekos.best](https://nekos.best/).


## Example
```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url: String = nekosbest::get(nekosbest::Category::Nekos).await?;
    println!("{}", img_url);
    Ok(())
}
```

Or with an amount(amount is capped at 20 by the server):

```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_urls: Vec<String> = nekosbest::get_amount(nekosbest::Category::Nekos, 20).await?;
    println!("{:?}", img_urls);
    Ok(())
}
```

Or if you already have a `reqwest::Client` that you want to use, use `get_with_client` and `get_with_client_amount` respectively.
