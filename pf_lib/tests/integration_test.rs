#[cfg(test)]
mod tests {
    use pf_lib::{find, FinderConfig};
    use serde_json::json;
    use std::process::Command;

    #[test]
    fn test_find() {
        let mut server = mockito::Server::new();

        let url = server.url();

        let body = json!([{"source_url": format!("{}/wp-content/uploads/2021/01/image.jpg", url)}, {"_embedded": {"wp:featuredmedia": [{"source_url": format!("{}/wp-content/uploads/2021/01/video.mp4", url)}]}}]);

        let api_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body(body.to_string())
            .create();

        let image_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/image.mp4")
            .with_header("Content-Type", "video/mp4")
            .create();

        let video_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/video.mp4")
            .with_header("Content-Type", "video/mp4")
            .create();

        let config = FinderConfig {
            url: url.clone(),
            ..Default::default()
        };

        let urls = find(&config).collect::<Vec<_>>();

        assert!(urls.contains(&format!("{}/wp-content/uploads/2021/01/image.mp4", url)));
        assert!(urls.contains(&format!("{}/wp-content/uploads/2021/01/video.mp4", url)));

        api_mock.assert();
        image_mock.assert();
        video_mock.assert();
    }

    #[test]
    fn test_find_with_no_urls() {
        let mut server = mockito::Server::new();

        let api_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body("[]")
            .create();

        let config = FinderConfig {
            url: server.url(),
            ..Default::default()
        };

        assert!(find(&config).next().is_none());
        api_mock.assert();
    }

    #[test]
    fn test_find_with_invalid_urls() {
        let mut server = mockito::Server::new();

        let url = server.url();

        let body = json!([{"source_url": format!("{}/wp-content/uploads/2021/01/image.jpg", url)}, {"_embedded": {"wp:featuredmedia": [{"source_url": format!("{}/wp-content/uploads/2021/01/video.mp4", url)}]}}]);

        let api_mock = server
            .mock("GET", "/wp-json/wp/v2/media?per_page=100")
            .with_body(body.to_string())
            .create();

        let not_found_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/image.mp4")
            .with_header("Content-Type", "video/mp4")
            .with_status(404)
            .create();

        let invalid_header_mock = server
            .mock("HEAD", "/wp-content/uploads/2021/01/video.mp4")
            .with_header("Content-Type", "application/json")
            .create();

        let config = FinderConfig {
            url,
            ..Default::default()
        };

        assert!(find(&config).next().is_none());

        api_mock.assert();
        not_found_mock.assert();
        invalid_header_mock.assert();
    }

    #[test]
    fn test_find_with_network_error() {
        let config = FinderConfig {
            url: "http:/example.com".to_string(),
            ..Default::default()
        };

        assert!(find(&config).next().is_none());
    }

    #[test]
    fn test_compile_fail_with_no_features() {
        let output = Command::new("cargo")
            .arg("build")
            .arg("--no-default-features")
            .output()
            .expect("Failed to execute cargo build");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error: At least one feature must be enabled."));
    }

    #[test]
    fn test_compile_success_with_features() {
        let output = Command::new("cargo")
            .arg("build")
            .arg("--features")
            .arg("mp4")
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success());
    }
}
