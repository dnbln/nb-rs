# `nb-blocking-util`

Procedural macro utility for removing `async` / `await` code from a function, assuming the interfaces have both an async and a blocking version of the functions called.

For example, `reqwest`'s `reqwest::Client` and `reqwest::blocking::Client` expose a very similar interface, so often it is enough to just remove `.await`s.

## Example usage

Let's say you have a function that fetches `https://rust-lang.org/`, by using a `reqwest` client:

```rust
async fn fetch_rust_lang(client: &reqwest::Client) -> Result<String, reqwest::Error> {
    Ok(client.get("https://rust-lang.org/")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}
```

And now you want to implement the same function if `#[cfg(feature = "blocking")]`, but blocking, gating the previous behind `#[cfg(not(feature = "blocking"))]`

Of course you can do that, but this macro
may help you get rid of duplicates like that:

```rust
// First expose a ReqwestClient to the function.
#[cfg(not(feature = "blocking"))]
type ReqwestClient = reqwest::Client;
#[cfg(feature = "blocking")]
type ReqwestClient = reqwest::blocking::Client;

// Then the function
async fn fetch_rust_lang(client: &ReqwestClient) -> Result<String, reqwest::Error> {
    Ok(client.get("https://rust-lang.org/")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}
```

And now everything left is put it behind
the `#[blocking]` attribute if `#[cfg(feature = "blocking")]`:

```rust
#[cfg_attr(feature = "blocking", nb_blocking_util::blocking)]
async fn fetch_rust_lang(client: &ReqwestClient) -> Result<String, reqwest::Error> {
    Ok(client.get("https://rust-lang.org/")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}
```