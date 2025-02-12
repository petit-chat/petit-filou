use crate::api;
use crate::link_utils;
use crate::url_extractor;
use crate::FinderConfig;

use async_stream::try_stream;
use futures_core::Stream;
use futures_util::pin_mut;
use futures_util::StreamExt;

/// Creates an asynchronous stream that fetches video URLs based on the provided configuration.
///
/// This function fetches new URLs from the WordPress API. It ensures that the video URLs exist before returning them.
///
/// # Arguments
///
/// * `config` - The `FinderConfig`.
///
/// # Returns
///
/// A `futures_core::Stream` over existing video URLs.
///
/// # Examples
///
/// ```rust
/// use futures_util::pin_mut;
/// use futures_util::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let config = pf_lib::FinderConfig {
///         url: "http://example.com".to_string(),
///         ..Default::default()
///     };
///
///     let stream = pf_lib::find(&config);
///
///     pin_mut!(stream); // needed for iteration
///
///     while let Some(res) = stream.next().await {
///         match res {
///             Ok(url) => println!("{}", url),
///             Err(e) => eprintln!("{}", e),
///         }
///     }
/// }
/// ```
pub fn find(
    config: &FinderConfig,
) -> impl Stream<Item = Result<String, Box<dyn std::error::Error>>> + '_ {
    let client = reqwest::Client::new();
    try_stream! {
        let stream = api::get_stream(&client, config);
        pin_mut!(stream);
        while let Some(body) = stream.next().await {
            for url in url_extractor::Xtract::new(&body?).run() {
                if link_utils::does_link_exist(&client, &url).await {
                    yield url;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::pin_mut;
    use futures_util::StreamExt;
    use serde_json::json;

    #[tokio::test]
    async fn test_find() {
        let mut server = mockito::Server::new_async().await;

        let url = server.url();

        let body_page_1 = json!([{"source_url": format!("{}/wp-content/uploads/2021/01/image.jpg", url)}, {"_embedded": {"wp:featuredmedia": [{"source_url": format!("{}/wp-content/uploads/2021/01/video.mp4", url)}]}}]);

        let api_page_1_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_header(
                "link",
                format!(
                    r#"<{}/wp-json/wp/v2/media?per_page=100&page=2>; rel="next""#,
                    url
                )
                .as_str(),
            )
            .with_body(body_page_1.to_string())
            .create_async()
            .await;

        let image_page_1_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/image.mp4")
            .with_header("Content-Type", "video/mp4")
            .create_async()
            .await;

        let video_page_1_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/video.mp4")
            .with_header("Content-Type", "video/mp4")
            .create_async()
            .await;

        let body_page_2 = json!([{"source_url": format!("{}/wp-content/uploads/2022/02/image.jpg", url)}, {"_embedded": {"wp:featuredmedia": [{"source_url": format!("{}/wp-content/uploads/2022/02/video.mp4", url)}]}}]);

        let api_page_2_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100&page=2")
            .with_body(body_page_2.to_string())
            .create_async()
            .await;

        let image_page_2_mock = server
            .mock("HEAD", "/wp-content/uploads/2022/02/image.mp4")
            .with_header("Content-Type", "video/mp4")
            .create_async()
            .await;

        let video_page_2_mock = server
            .mock("HEAD", "/wp-content/uploads/2022/02/video.mp4")
            .with_header("Content-Type", "video/mp4")
            .create_async()
            .await;

        let config = FinderConfig {
            url: url.clone(),
            ..Default::default()
        };

        let stream = find(&config);
        pin_mut!(stream);

        let urls = stream
            .filter_map(|res| async { res.ok() })
            .collect::<Vec<_>>()
            .await;

        api_page_1_mock.assert_async().await;
        image_page_1_mock.assert_async().await;
        video_page_1_mock.assert_async().await;
        api_page_2_mock.assert_async().await;
        image_page_2_mock.assert_async().await;
        video_page_2_mock.assert_async().await;

        assert!(urls.contains(&format!("{}/wp-content/uploads/2021/01/image.mp4", url)));
        assert!(urls.contains(&format!("{}/wp-content/uploads/2021/01/video.mp4", url)));
        assert!(urls.contains(&format!("{}/wp-content/uploads/2022/02/image.mp4", url)));
        assert!(urls.contains(&format!("{}/wp-content/uploads/2022/02/video.mp4", url)));
    }

    #[tokio::test]
    async fn test_find_with_no_urls() {
        let mut server = mockito::Server::new_async().await;

        let api_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body("[]")
            .create_async()
            .await;

        let config = FinderConfig {
            url: server.url(),
            ..Default::default()
        };

        let stream = find(&config);
        pin_mut!(stream);
        let next = stream.next().await;

        api_mock.assert_async().await;

        assert!(next.is_none());
    }

    #[tokio::test]
    async fn test_find_with_invalid_urls() {
        let mut server = mockito::Server::new_async().await;

        let url = server.url();

        let body = json!([{"source_url": format!("{}/wp-content/uploads/2021/01/image.jpg", url)}, {"_embedded": {"wp:featuredmedia": [{"source_url": format!("{}/wp-content/uploads/2021/01/video.mp4", url)}]}}]);

        let api_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body(body.to_string())
            .create_async()
            .await;

        let not_found_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/image.mp4")
            .with_header("Content-Type", "video/mp4")
            .with_status(404)
            .create_async()
            .await;

        let invalid_header_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/video.mp4")
            .with_header("Content-Type", "application/json")
            .create_async()
            .await;

        let config = FinderConfig {
            url,
            ..Default::default()
        };

        let stream = find(&config);
        pin_mut!(stream);
        let next = stream.next().await;

        api_mock.assert_async().await;
        not_found_mock.assert_async().await;
        invalid_header_mock.assert_async().await;

        assert!(next.is_none());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_find_with_invalid_json() {
        let mut server = mockito::Server::new_async().await;

        server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body("invalid json")
            .create_async()
            .await;

        let config = FinderConfig {
            url: server.url(),
            ..Default::default()
        };

        let stream = find(&config);

        pin_mut!(stream);

        stream.next().await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_find_with_non_wordpress_json() {
        let mut server = mockito::Server::new_async().await;

        server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body("{}")
            .create_async()
            .await;

        let config = FinderConfig {
            url: server.url(),
            ..Default::default()
        };

        let stream = find(&config);

        pin_mut!(stream);

        stream.next().await;
    }
}
