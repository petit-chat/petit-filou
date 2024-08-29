use crate::{link_utils, FinderConfig};

use reqwest::blocking::Client;
use serde_json::Value;
use std::rc::Rc;

/// The `Api` struct is responsible for interacting with WordPress API.
///
/// It uses a `reqwest` client to fetch pages and keeps track of the next link to fetch.
pub struct Api {
    client: Rc<Client>,
    next_link: Option<String>,
}

impl Api {
    /// Creates a new `Api` instance.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference-counted `reqwest::blocking::Client` used to make HTTP requests.
    /// * `config` - The `FinderConfig` used to build the WordPress API URL.
    ///
    /// # Returns
    ///
    /// A new instance of `Api`.
    pub fn new(client: Rc<Client>, config: &FinderConfig) -> Self {
        Self {
            client,
            next_link: link_utils::build_url_from_config(config).ok(),
        }
    }
}

impl Iterator for Api {
    type Item = Value;

    /// Fetches the next page from the WordPress API.
    ///
    /// # Returns
    ///
    /// An `Option` containing the JSON response if the request was successful, or `None` if there
    /// are no more pages or if an error occurred.
    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("Fetching next page from API");

        let url = self.next_link.take()?;

        let response = self.client.get(url).send().ok()?;

        let headers = response.headers();
        self.next_link = link_utils::get_next_link_from_headers(headers);

        let json = response.json::<serde_json::Value>().ok()?;
        Some(json)
    }
}
