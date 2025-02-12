//! # pf_lib
//!
//! This crate provides functionality to retrieve existing video URLs from a WordPress websites that use
//! the REST API. It supports both [`media`] and [`posts`] resources.
//!
//! ## Usage
//!
//! ```rust
//! use futures_util::pin_mut;
//! use futures_util::stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = pf_lib::FinderConfig {
//!         url: "http://example.com".to_string(),
//!         ..Default::default()
//!     };
//!
//!     let stream = pf_lib::find(&config);
//!
//!     pin_mut!(stream); // needed for iteration
//!
//!     while let Some(res) = stream.next().await {
//!         match res {
//!             Ok(url) => println!("{}", url),
//!             Err(e) => eprintln!("{}", e),
//!         }
//!     }
//! }
//! ```
//!
//! [`media`]: https://developer.wordpress.org/rest-api/reference/media/
//! [`posts`]: https://developer.wordpress.org/rest-api/reference/posts/

mod api;
mod config;
mod finder;
mod link_utils;
mod mime_types;
mod url_extractor;

pub use config::{FinderConfig, FinderTarget};
pub use finder::find;
