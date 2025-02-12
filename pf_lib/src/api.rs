use crate::{link_utils, FinderConfig};

use async_stream::try_stream;
use futures_core::Stream;

/// This function takes an HTTP client and a configuration object, and returns a stream of JSON values. It
/// repeatedly makes GET requests to the API, following pagination links found in the response headers.
///
/// # Arguments
///
/// * `client` - `reqwest::Client` used to make HTTP requests.
/// * `config` - The `FinderConfig`.
///
/// # Returns
///
/// An asynchronous `futures_core::Stream` of `Result` containing response `String` body if successful, or
/// an error.
pub fn get_stream<'a>(
    client: &'a reqwest::Client,
    config: &'a FinderConfig,
) -> impl Stream<Item = Result<String, Box<dyn std::error::Error>>> + 'a {
    let mut next_link = link_utils::build_url_from_config(config).ok();
    try_stream! {
        while let Some(url) = next_link {
            let response = client.get(url).send().await.map_err(|e| {
                format!("Failed to send request: {}", e)
            })?;

            if !response.status().is_success() {
                Err(format!("Failed to fetch URL, status code: {}", response.status()))?;
            }

            let headers = response.headers();
            next_link = link_utils::get_next_link_from_headers(headers);

            let body = response.text().await.map_err(|e| {
                format!("Failed to read response body: {}", e)
            })?;

            yield body;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::pin_mut;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_get_stream_with_404_api_url() {
        let mut server = mockito::Server::new_async().await;

        let api_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_status(404)
            .create_async()
            .await;

        let client = reqwest::Client::new();
        let config = FinderConfig {
            url: server.url(),
            ..Default::default()
        };

        let stream = get_stream(&client, &config);

        pin_mut!(stream);

        let next = stream.next().await.unwrap();

        api_mock.assert_async().await;

        assert!(next.is_err());

        let error = next.err().unwrap();
        assert_eq!(
            error.to_string(),
            "Failed to fetch URL, status code: 404 Not Found"
        );
    }

    #[tokio::test]
    async fn test_get_stream_with_invalid_url() {
        let client = reqwest::Client::new();
        let config = FinderConfig {
            url: "http://examplecom".to_string(),
            ..Default::default()
        };

        let stream = get_stream(&client, &config);

        pin_mut!(stream);

        let next = stream.next().await.unwrap();

        assert!(next.is_err());

        let error = next.err().unwrap();
        assert_eq!(
            error.to_string(),
            "Failed to send request: error sending request for url (http://examplecom/wp-json/wp/v2/media?per_page=100)"
        );
    }
}
