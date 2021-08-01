# nekosbest

Rust API wrapper for [nekos.best](https://nekos.best/).

## Usage
```toml
[dependencies]
nekosbest = "0.4"
```

## Example
```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url: String = nekosbest::get(nekosbest::Category::Nekos).await?.url;
    println!("{}", img_url);
    Ok(())
}
```

Or with an amount(amount is capped at 20 by the server):

```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_urls: Vec<String> = nekosbest::get_amount(nekosbest::Category::Nekos, 20).await?.url;
    println!("{:?}", img_urls);
    Ok(())
}
```

Or if you already have a `reqwest::Client` that you want to use, use `get_with_client` and `get_with_client_amount` respectively.

With Category::Nekos, there is another property called details:

```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get(nekosbest::Category::Nekos).await?.details.unwrap();
    println!("Source: {}", details.source_url);
    println!("Artist: {}", details.artist_name);
    println!("Artist link: {}", details.artist_href);
    Ok(())
}
```

By using the `local` feature, you can completelly skip requests to the API.
```rust,no_run
fn main() {
    let img_url = nekosbest::local::Nekos.get();
    println!("{}", img_url);
    Ok(())
}
```

Or if you have your own random number:
```rust,no_run
fn main() {
    let your_random = unimplemented!();
    let img_url = nekosbest::local::Nekos.get_random(your_random);
    println!("{}", img_url);
    Ok(())
}
```

Take a look at [the build script](build.rs) and [src/local.rs](src/local.rs) if
you want to find out how it works.