use crate::api::Api;
use crate::link_utils;
use crate::url_extractor;
use crate::FinderConfig;

use reqwest::blocking::Client;
use std::collections::HashSet;
use std::rc::Rc;

/// The `Finder` struct is responsible for fetching URLs from a WordPress API and caching them.
struct Finder {
    client: Rc<Client>,
    urls: Vec<String>,
    api: Api,
}

impl Finder {
    /// Creates a new `Finder` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The `FinderConfig`.
    ///
    /// # Returns
    ///
    /// A new instance of `Finder`.
    pub fn new(config: &FinderConfig) -> Self {
        let client = Rc::new(Client::new());
        let api = Api::new(Rc::clone(&client), config);
        Self {
            api,
            client,
            urls: vec![],
        }
    }
}

impl Iterator for Finder {
    type Item = String;

    /// Fetches the next video URL.
    ///
    /// This method first checks the cached video URLs. If no cached video URLs are available, it fetches
    /// new URLs from the WordPress API. It ensures that the video URLs exist before returning them.
    ///
    /// # Returns
    ///
    /// An `Option` containing the next video URL if available, or `None` if no more video URLs are available.
    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("Fetching next URL");

        if let Some(url) = self.urls.pop() {
            log::debug!("Checking cached URL for existence...");

            if link_utils::does_link_exist(&self.client, &url) {
                log::info!("Found URL: {}", url);
                return Some(url);
            }

            log::info!("URL does not exist: {}", url);

            return self.next();
        }

        log::debug!("No cached URLs, fetching from API...");

        if let Some(value) = self.api.next() {
            let values = value.as_array()?;

            let urls: HashSet<String> = url_extractor::p0(values)
                .into_iter()
                .chain(url_extractor::p1(values))
                .chain(url_extractor::p2(values))
                .chain(url_extractor::p3(values))
                .collect();

            log::debug!("Extracted {} URL(s).", urls.len());

            self.urls = urls.into_iter().collect();

            return self.next();
        }

        log::debug!("No more URLs from API");

        None
    }
}

/// Finds video URLs based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The `FinderConfig`.
///
/// # Returns
///
/// An iterator over existing video URLs.
///
/// # Examples
///
/// ```
/// use pf_lib::FinderConfig;
///
/// let config = FinderConfig {
///     url: "http://example.com".to_string(),
///     ..Default::default()
/// };
///
/// for url in pf_lib::find(&config) {
///     println!("{}", url);
/// }
/// ```
pub fn find(config: &FinderConfig) -> impl Iterator<Item = String> {
    Finder::new(config)
}
