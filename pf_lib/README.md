# pf_lib

[![crates.io version](https://img.shields.io/crates/v/pf_lib.svg)](https://crates.io/crates/pf_lib)

A Rust library to retrieve WordPress MP4 videos. Supports filtering by date and including/excluding specific IDs, categories, and tags.

## Example

```rust
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use pf_lib::{FinderConfig, FinderTarget};

let config = FinderConfig {
    url: "http://example.com".to_string(),
    target: FinderTarget::Media,
    ..Default::default()
};

let stream = pf_lib::find(&config);

pin_mut!(stream); // needed for iteration

while let Some(res) = stream.next().await {
    match res {
        Ok(url) => println!("{}", url),
        Err(e) => eprintln!("{}", e),
    }
}
```
