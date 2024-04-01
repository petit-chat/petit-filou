use mockito::{Matcher, Server};
use petit_filou::{Config, Target};
use serde_json::json;

#[tokio::test]
async fn test_run_with_target_posts() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let m1 = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_header(
            "Link",
            &format!("<{}/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia&page=2>; rel=\"next\"", base_url),
        )
        .with_body(json!([
            {
                "_embedded": {
                    "wp:featuredmedia": [
                        {
                            "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                        }
                    ]
                },
            }
        ]).to_string())
        .create_async()
        .await;

    let m2 = server
        .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
        .create_async()
        .await;

    let m3 = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
            Matcher::UrlEncoded("page".into(), "2".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_header("Link", &format!("<{}/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia&page=1>; rel=\"prev\", <{}/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia&page=3>; rel=\"next\"", base_url, base_url))
        .with_body(json!([
            {
                "content": {
                    "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                },
                "excerpt": {
                    "rendered": format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur : {}/blog/wp-content/uploads/2001/01/video.mp4. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", base_url)
                },
            }
        ]).to_string())
        .create_async()
        .await;

    let m4 = server
        .mock("HEAD", "/blog/wp-content/uploads/2001/01/video.mp4")
        .create_async()
        .await;

    let m5 = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
            Matcher::UrlEncoded("page".into(), "3".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_header(
            "Link",
            &format!("<{}/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia&page=2>; rel=\"prev\"", base_url),
        )
        .with_body(
            json!([
            {
                "date": "2000-01-01T00:00:00",
                "link": format!("{}/blog/slug/", base_url),
            }
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let m6 = server
        .mock("HEAD", "/blog/wp-content/uploads/2000/01/slug.mp4")
        .create_async()
        .await;

    let config = Config {
        url: base_url,
        target: Target::Posts {
            categories_exclude: Vec::new(),
            tags_exclude: Vec::new(),
        },
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let urls = petit_filou::run(&config).await.unwrap();

    m1.assert_async().await;
    m2.assert_async().await;
    m3.assert_async().await;
    m4.assert_async().await;
    m5.assert_async().await;
    m6.assert_async().await;

    assert_eq!(urls.len(), 3);
}

#[tokio::test]
async fn test_run_with_target_media() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let m1 = server
        .mock("GET", "/wp-json/wp/v2/media")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "per_page".into(),
            "100".into(),
        )]))
        .with_header("content-type", "application/json")
        .with_header(
            "Link",
            &format!(
                "<{}/wp-json/wp/v2/media?per_page=100&page=2>; rel=\"next\"",
                base_url
            ),
        )
        .with_body(
            json!([
                {
                    "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.png", base_url)
                }
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let m2 = server
        .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
        .create_async()
        .await;

    let m3 = server
        .mock("GET", "/wp-json/wp/v2/media")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("page".into(), "2".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_header("Link", &format!("<{}/wp-json/wp/v2/media?per_page=100&page=1>; rel=\"prev\", <{}/wp-json/wp/v2/media?per_page=100&page=3>; rel=\"next\"", base_url, base_url))
        .with_body(json!([
            {
                "source_url": format!("{}/wp-content/uploads/2003/03/image.jpeg", base_url)
            }
        ]).to_string())
        .create_async()
        .await;

    let m4 = server
        .mock("HEAD", "/wp-content/uploads/2003/03/image.mp4")
        .with_status(404)
        .create_async()
        .await;

    let m5 = server
        .mock("GET", "/wp-json/wp/v2/media")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("page".into(), "3".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_header(
            "Link",
            &format!("<{}/wp-json/wp/v2/posts?per_page=100&page=2>; rel=\"prev\"", base_url),
        )
        .with_body(
            json!([
            {
                "date": "2000-01-01T00:00:00",
                "link": format!("{}/docs/astra-widget-translation-with-polylang/attachment/slug/", base_url),
            }
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let m6 = server
        .mock("HEAD", "/wp-content/uploads/2000/01/slug.mp4")
        .create_async()
        .await;

    let config = Config {
        url: base_url.clone(),
        target: Target::Media,
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let urls = petit_filou::run(&config).await.unwrap();

    m1.assert_async().await;
    m2.assert_async().await;
    m3.assert_async().await;
    m4.assert_async().await;
    m5.assert_async().await;
    m6.assert_async().await;

    assert_eq!(urls.len(), 2);
    assert!(urls.contains(&format!("{}/wp-content/uploads/2000/01/slug.mp4", base_url)));
    assert!(urls.contains(&format!(
        "{}/blog/wp-content/uploads/2002/02/image.mp4",
        base_url
    )));
    assert!(!urls.contains(&format!(
        "{}/wp-content/uploads/2003/03/image.mp4",
        base_url
    )));
}

#[tokio::test]
async fn test_run_with_parameters() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let m1 = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
            Matcher::UrlEncoded("categories_exclude".into(), "1,2".into()),
            Matcher::UrlEncoded("tags_exclude".into(), "3,4".into()),
            Matcher::UrlEncoded("before".into(), "2001-01-01T00:00:00".into()),
            Matcher::UrlEncoded("modified_before".into(), "2002-02-02T00:00:00".into()),
            Matcher::UrlEncoded("after".into(), "2003-03-03T00:00:00".into()),
            Matcher::UrlEncoded("modified_after".into(), "2004-04-04T00:00:00".into()),
            Matcher::UrlEncoded("exclude".into(), "5,6".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_body(json!([
            {
                "_embedded": {
                    "wp:featuredmedia": [
                        {
                            "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                        }
                    ]
                },
            }
        ]).to_string())
        .create_async()
        .await;

    let m2 = server
        .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
        .create_async()
        .await;

    let config = Config {
        url: base_url,
        target: Target::Posts {
            categories_exclude: vec![1, 2],
            tags_exclude: vec![3, 4],
        },
        before: Some(String::from("2001-01-01T00:00:00")),
        modified_before: Some(String::from("2002-02-02T00:00:00")),
        after: Some(String::from("2003-03-03T00:00:00")),
        modified_after: Some(String::from("2004-04-04T00:00:00")),
        exclude: vec![5, 6],
    };

    let urls = petit_filou::run(&config).await.unwrap();

    m1.assert_async().await;
    m2.assert_async().await;

    assert!(!urls.is_empty());
}

#[tokio::test]
async fn test_run_with_network_error() {
    let config = Config {
        url: String::from("http://fake_server"),
        target: Target::Posts {
            categories_exclude: Vec::new(),
            tags_exclude: Vec::new(),
        },
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let result = petit_filou::run(&config).await;

    assert!(result.is_err(), "Expected a network error result");
}

#[tokio::test]
async fn test_run_with_api_http_error() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let mock = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
        ]))
        .with_status(404)
        .create_async()
        .await;

    let config = Config {
        url: base_url,
        target: Target::Posts {
            categories_exclude: Vec::new(),
            tags_exclude: Vec::new(),
        },
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let result = petit_filou::run(&config).await;

    mock.assert_async().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_run_with_invalid_json() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let mock = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_header(
            "Link",
            &format!("<{}/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia&page=2>; rel=\"next\"", base_url),
        )
        .with_body("hello world")
        .create_async()
        .await;

    let config = Config {
        url: base_url,
        target: Target::Posts {
            categories_exclude: Vec::new(),
            tags_exclude: Vec::new(),
        },
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let result = petit_filou::run(&config).await;

    mock.assert_async().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_run_handles_empty_json() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let mock = server
        .mock("GET", "/wp-json/wp/v2/media")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "per_page".into(),
            "100".into(),
        )]))
        .with_header("content-type", "application/json")
        .with_body(json!([{}]).to_string())
        .create_async()
        .await;

    let config = Config {
        url: base_url.clone(),
        target: Target::Media,
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let urls = petit_filou::run(&config).await.unwrap();

    mock.assert_async().await;

    assert!(urls.is_empty());
}

#[tokio::test]
async fn test_run_empty_link_header() {
    let mut server = Server::new_async().await;

    let base_url = server.url();

    let m1 = server
        .mock("GET", "/wp-json/wp/v2/posts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("per_page".into(), "100".into()),
            Matcher::UrlEncoded("_embed".into(), "wp:featuredmedia".into()),
        ]))
        .with_header("content-type", "application/json")
        .with_body(json!([
            {
                "_embedded": {
                    "wp:featuredmedia": [
                        {
                            "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                        }
                    ]
                },
            }
        ]).to_string())
        .create_async()
        .await;

    let m2 = server
        .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
        .create_async()
        .await;

    let config = Config {
        url: base_url,
        target: Target::Posts {
            categories_exclude: Vec::new(),
            tags_exclude: Vec::new(),
        },
        before: None,
        modified_before: None,
        after: None,
        modified_after: None,
        exclude: Vec::new(),
    };

    let urls = petit_filou::run(&config).await.unwrap();

    m1.assert_async().await;
    m2.assert_async().await;

    assert!(!urls.is_empty());
}
