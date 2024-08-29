//! This library provides functionality to find and fetch existing video URLs from a WordPress website.
//!
//! The main components of this library are:
//! - `api`: Handles interactions with the paginated WordPress API.
//! - `config`: Defines configuration options for the `Finder`.
//! - `finder`: Implements the logic to find and fetch existing video URLs.
//! - `link_utils`: Utility functions for handling links.
//! - `url_extractor`: Functions to extract URLs from WordPress API responses.

mod api;
mod config;
mod finder;
mod link_utils;
mod url_extractor;

pub use config::{FinderConfig, FinderTarget};
pub use finder::find;
