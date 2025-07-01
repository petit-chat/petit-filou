use crate::{mime_types::SUPPORTED_MIME_TYPES, FinderConfig, FinderTarget};

use std::error::Error;

/// Builds a paginated WordPress API URL from the given `FinderConfig`.
///
/// # Arguments
///
/// * `config` - A reference to the `FinderConfig` struct containing the configuration.
///
/// # Returns
///
/// A `Result` containing the constructed URL as a `String` if successful, or an error if the URL is empty.
pub fn build_url_from_config(config: &FinderConfig) -> Result<String, Box<dyn Error>> {
    if config.url.is_empty() {
        return Err("URL is required".into());
    }

    Ok(format!(
        "{}/wp-json/wp/v2/{}?per_page=100{}{}{}{}{}{}",
        config.url,
        match &config.target {
            FinderTarget::Posts { .. } => "posts",
            FinderTarget::Media => "media",
        },
        match &config.before {
            Some(value) => format!("&before={value}"),
            None => String::new(),
        },
        match &config.modified_before {
            Some(value) => format!("&modified_before={value}"),
            None => String::new(),
        },
        match &config.after {
            Some(value) => format!("&after={value}"),
            None => String::new(),
        },
        match &config.modified_after {
            Some(value) => format!("&modified_after={value}"),
            None => String::new(),
        },
        if !&config.exclude.is_empty() {
            format!(
                "&exclude={}",
                config
                    .exclude
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        } else {
            String::new()
        },
        match &config.target {
            FinderTarget::Posts {
                categories_exclude,
                tags_exclude,
            } => format!(
                "&_embed=wp:featuredmedia{}{}",
                if !categories_exclude.is_empty() {
                    format!(
                        "&categories_exclude={}",
                        categories_exclude
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                } else {
                    String::new()
                },
                if !tags_exclude.is_empty() {
                    format!(
                        "&tags_exclude={}",
                        tags_exclude
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                } else {
                    String::new()
                },
            ),
            _ => String::new(),
        }
    ))
}

/// Extracts the next link from the HTTP headers.
///
/// This function looks for the `link` header in the provided header map and extracts the URL
/// for the next page if it exists.
///
/// # Arguments
///
/// * `headers` - A reference to the `reqwest::header::HeaderMap` containing the HTTP headers.
///
/// # Returns
///
/// An `Option` containing the next link as a `String` if it exists, or `None` if it does not.
pub fn get_next_link_from_headers(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get("link")
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(',').find(|text| text.contains("rel=\"next\"")))
        .and_then(|next| next.split(';').next())
        .map(|res| {
            res.trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_string()
        })
}

/// Checks if a link exists by sending a HEAD request.
///
/// This function sends a HEAD request to the given URL and checks if the response status
/// indicates success.
///
/// # Arguments
///
/// * `client` - A reference to the `reqwest::blocking::Client` used to send the request.
/// * `url` - The URL to check.
///
/// # Returns
///
/// `true` if the link exists (i.e., the response status is successful), `false` otherwise.
pub async fn does_link_exist(client: &reqwest::Client, url: &str) -> bool {
    match client.head(url).send().await {
        Ok(response) => {
            response.status().is_success()
                && response.headers().get("content-type").is_some_and(|v| {
                    SUPPORTED_MIME_TYPES
                        .iter()
                        .map(|(_, mime)| *mime)
                        .collect::<Vec<_>>()
                        .contains(&v.to_str().unwrap_or_default())
                })
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_from_config() {
        let config = FinderConfig {
            after: Some("2021-01-01T00:00:00".to_string()),
            before: Some("2022-02-02T00:00:00".to_string()),
            exclude: vec![1, 2, 3],
            modified_after: Some("2023-03-03T00:00:00".to_string()),
            modified_before: Some("2024-04-04T00:00:00".to_string()),
            target: FinderTarget::Posts {
                categories_exclude: vec![4, 5, 6],
                tags_exclude: vec![7, 8, 9],
            },
            url: "http://example.com".to_string(),
        };

        let url = build_url_from_config(&config).unwrap();

        assert_eq!(
            url,
            "http://example.com/wp-json/wp/v2/posts?per_page=100&before=2022-02-02T00:00:00&modified_before=2024-04-04T00:00:00&after=2021-01-01T00:00:00&modified_after=2023-03-03T00:00:00&exclude=1,2,3&_embed=wp:featuredmedia&categories_exclude=4,5,6&tags_exclude=7,8,9"
        );
    }

    #[test]
    fn test_build_url_from_config_with_default_config() {
        let config = FinderConfig {
            url: "http://example.com".to_string(),
            ..Default::default()
        };

        let url = build_url_from_config(&config).unwrap();

        assert_eq!(url, "http://example.com/wp-json/wp/v2/media?per_page=100");
    }

    #[test]
    fn test_build_url_from_config_with_posts_target_and_empty_fields() {
        let config = FinderConfig {
            url: "http://example.com".to_string(),
            target: FinderTarget::Posts {
                categories_exclude: vec![],
                tags_exclude: vec![],
            },
            ..Default::default()
        };

        let url = build_url_from_config(&config).unwrap();

        assert_eq!(
            url,
            "http://example.com/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia"
        );
    }

    #[test]
    fn test_build_url_from_config_with_invalid_config() {
        let config = FinderConfig {
            ..Default::default()
        };

        let res = build_url_from_config(&config);

        assert!(res.is_err());
    }

    #[test]
    fn test_get_next_link_from_headers() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "link",
            reqwest::header::HeaderValue::from_str(
                r#"<http://example.com/wp-json/wp/v2/posts?page=2>; rel="next""#,
            )
            .unwrap(),
        );

        let next_link = get_next_link_from_headers(&headers);
        assert_eq!(
            next_link,
            Some("http://example.com/wp-json/wp/v2/posts?page=2".to_string())
        );
    }

    #[test]
    fn test_get_next_link_from_headers_with_no_next() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "link",
            reqwest::header::HeaderValue::from_str(
                r#"<http://example.com/wp-json/wp/v2/posts?page=2>; rel="prev""#,
            )
            .unwrap(),
        );

        let next_link = get_next_link_from_headers(&headers);

        assert_eq!(next_link, None);
    }

    #[tokio::test]
    async fn test_does_link_exist() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("HEAD", "/")
            .with_header("Content-Type", "video/mp4")
            .create_async()
            .await;

        let client = reqwest::Client::new();
        let url = server.url();

        let exists = does_link_exist(&client, &url).await;

        mock.assert_async().await;
        assert!(exists);
    }

    #[tokio::test]
    async fn test_does_link_exist_when_incorrect_header() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("HEAD", "/")
            .with_header("Content-Type", "text/html; charset=UTF-8")
            .create_async()
            .await;

        let client = reqwest::Client::new();
        let url = server.url();

        let exists = does_link_exist(&client, &url).await;

        mock.assert_async().await;
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_does_link_exist_when_url_does_not_exist() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("HEAD", "/")
            .with_header("Content-Type", "video/mp4")
            .with_status(503)
            .create_async()
            .await;

        let client = reqwest::Client::new();
        let url = server.url();

        let exists = does_link_exist(&client, &url).await;

        mock.assert_async().await;
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_does_link_exist_with_network_error() {
        let client = reqwest::Client::new();
        let url = "http://example";
        let exists = does_link_exist(&client, url).await;
        assert!(!exists);
    }
}
