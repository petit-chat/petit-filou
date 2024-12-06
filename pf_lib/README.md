# pf_lib

A Rust library to retrieve WordPress MP4 videos. Supports filtering by date and including/excluding specific IDs, categories, and tags.

## Example

```rust
use pf_lib::{FinderConfig, FinderTarget};

let config = FinderConfig {
    url: "http://example.com".to_string(),
    target: FinderTarget::Media,
    ..Default::default()
};

for url in pf_lib::find(&config) {
    println!("{}", url);
}
```
