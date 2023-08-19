# nekosbest

Rust API wrapper for [nekos.best](https://nekos.best/).

## Usage

```toml
[dependencies]
nekosbest = "0.20"
```

## Example

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url: String = nekosbest::get(nekosbest::Category::Neko).await?.url;
    println!("{img_url}");
    Ok(())
}
```

Or with an amount (amount is capped at 20 by the server):

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let images = nekosbest::get_amount(nekosbest::Category::Neko, 20).await?.0;
    println!("{images:?}");
    Ok(())
}
```

Or if you already have a `Client` that you want to use,
use `get_with_client` and `get_with_client_amount` respectively:

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = nekosbest::client::Client::new(nekosbest::client::ClientConfig::default());
    let details = nekosbest::get_with_client(&client, nekosbest::Category::Neko)
        .await?
        .details
        .try_into_image()
        .unwrap();
    println!("Source: {}", details.source_url);
    println!("Artist: {}", details.artist_name);
    println!("Artist link: {}", details.artist_href);
    Ok(())
}
```

There is another property called `details`:

For `Category::Neko`, `Category::Husbando`, `Category::Kitsune`, `Category::Waifu` (image endpoints):

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get(nekosbest::Category::Neko)
        .await?
        .details
        .try_into_image()
        .unwrap();
    println!("Source: {}", details.source_url);
    println!("Artist: {}", details.artist_name);
    println!("Artist link: {}", details.artist_href);
    Ok(())
}
```

For everything else (GIF endpoints):

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::get(nekosbest::Category::Pat)
        .await?
        .details
        .try_into_gif()
        .unwrap();
    println!("Anime name: {}", details.anime_name);
    Ok(())
}
```

Or with the `strong-types` feature, bringing strong types guarantees for details, so no `unwrap` / `expect` for the details type:

**Warning**: Experimental, may change at any point in the future.

Remember to add the `st_` in front of `get`, `get_amount`, `get_with_client` and `get_with_client_amount`.

Neko:

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::st_get::<nekosbest::Neko>().await?;
    let details = resp.details();
    println!("Artist: {}", details.artist_name);
    println!("Artist link: {}", details.artist_href);
    println!("Source: {}", details.source_url);
    Ok(())
}
```

GIF:

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let details = nekosbest::st_get::<nekosbest::Pat>().await?.details;
    println!("Anime name: {}", details.anime_name);
    Ok(())
}
```

## Downloading the images.

With the `download` feature, you can download the images directly, like so:

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = nekosbest::get(nekosbest::Category::Neko).await?;
    let image = nekosbest::download::download(&resp).await?;
    
    // maybe also save, or just use it directly
    tokio::task::spawn_blocking(move || image.save("neko.png")).await??;

    // or alternatively, if you just want to save it, without
    // loading the whole image in-memory:
    nekosbest::download::download_to_file(&resp, "neko.png").await?;
    Ok(())
}
```

Or directly from a given URL:

```rust ,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = nekosbest::get(nekosbest::Category::Neko).await?;
    // let url: String = resp.url;
    let url = "https://nekos.best/api/v2/neko/1efcda2d-d0d3-4e96-9b40-86852374b4bc.png".to_owned();
    let image = nekosbest::download::download_from_url(&url).await?;
    tokio::task::spawn_blocking(move || image.save("neko.png")).await??;

    // or alternatively, if you just want to save it, without
    // loading the whole image in-memory:
    nekosbest::download::download_from_url_to_file(&url, "neko.png").await?;
    Ok(())
}
```

## Blocking client

All functions become blocking when used with the "blocking" feature.
