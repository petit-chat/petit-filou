pub mod args;

use std::collections::HashSet;

pub struct Config {
    pub url: String,
    pub target: Target,
    pub before: Option<String>,
    pub modified_before: Option<String>,
    pub after: Option<String>,
    pub modified_after: Option<String>,
    pub exclude: Vec<u16>,
}

pub enum Target {
    Posts {
        categories_exclude: Vec<u16>,
        tags_exclude: Vec<u16>,
    },
    Media,
}

pub async fn run(config: &Config) -> Result<HashSet<String>, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut next_link = Some(url::build(config));

    let mut ret: HashSet<String> = HashSet::new();

    while let Some(url) = next_link {
        let response = client.get(url).send().await?;
        let headers = response.headers();

        next_link = headers
            .get("link")
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split(',').find(|text| text.contains("rel=\"next\"")))
            .and_then(|next| next.split(';').next())
            .map(|res| {
                res.trim()
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_string()
            });

        let json: serde_json::Value = response.json().await?;

        let urls = url::find(&client, &json).await?;

        ret.extend(urls);
    }

    Ok(ret)
}

mod url {
    use crate::{Config, Target};
    use std::collections::HashSet;

    impl Target {
        fn value(&self) -> &str {
            match self {
                Target::Posts { .. } => "posts",
                Target::Media => "media",
            }
        }
    }

    pub fn build(config: &Config) -> String {
        format!(
            "{}/wp-json/wp/v2/{}?per_page=100{}{}{}{}{}{}",
            config.url,
            config.target.value(),
            match &config.before {
                Some(value) => format!("&before={}", value),
                None => String::new(),
            },
            match &config.modified_before {
                Some(value) => format!("&modified_before={}", value),
                None => String::new(),
            },
            match &config.after {
                Some(value) => format!("&after={}", value),
                None => String::new(),
            },
            match &config.modified_after {
                Some(value) => format!("&modified_after={}", value),
                None => String::new(),
            },
            if !&config.exclude.is_empty() {
                format!(
                    "&exclude={}",
                    &config
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
                Target::Posts {
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
        )
    }

    pub async fn find(
        client: &reqwest::Client,
        value: &serde_json::Value,
    ) -> Result<HashSet<String>, reqwest::Error> {
        let parsers: Vec<fn(&serde_json::Value) -> HashSet<String>> =
            vec![from::p0, from::p1, from::p2, from::p3];

        for parser in parsers {
            let parsed_urls = parser(value);
            if parsed_urls.is_empty() {
                continue;
            }

            let urls = filter(client, &parsed_urls).await?;
            if urls.is_empty() {
                continue;
            }

            return Ok(urls);
        }

        Ok(HashSet::new())
    }

    async fn filter(
        client: &reqwest::Client,
        urls: &HashSet<String>,
    ) -> Result<HashSet<String>, reqwest::Error> {
        let mut ret = HashSet::new();
        for url in urls {
            let response = client.head(url).send().await?;
            if response.error_for_status().is_ok() {
                ret.insert(url.clone());
            }
        }
        Ok(ret)
    }

    mod from {
        use lazy_static::lazy_static;
        use regex::Regex;
        use std::collections::HashSet;

        lazy_static! {
            static ref SRC_RE: Regex =
                Regex::new(r"^https?://[^/]+(?:/blog)?/wp-content/uploads/\d{4}/\d{2}/[^/]+\.\w+$")
                    .unwrap();
            static ref EXT_RE: Regex = Regex::new(r"\.\w+$").unwrap();
            static ref BODY_RE: Regex =
                Regex::new(r"https?://[^/]+(?:/blog)?/wp-content/uploads/\d{4}/\d{2}/[^/]+\.mp4")
                    .unwrap();
            static ref LINK_RE: Regex =
                Regex::new(r"^(https?://[^/]+)(?:/[^/]+)*/([^/]+)/?$").unwrap();
            static ref DATE_RE: Regex =
                Regex::new(r"^(\d{4})-(\d{2})-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap();
        }

        pub(super) fn p0(value: &serde_json::Value) -> HashSet<String> {
            value
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|media| media["source_url"].as_str())
                .filter(|&url| SRC_RE.is_match(url))
                .map(|url| EXT_RE.replace(url, ".mp4").to_string())
                .collect()
        }

        pub(super) fn p1(value: &serde_json::Value) -> HashSet<String> {
            value
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|post| post["_embedded"]["wp:featuredmedia"].as_array())
                .flatten()
                .filter_map(|media| media["source_url"].as_str())
                .filter(|&url| SRC_RE.is_match(url))
                .map(|url| EXT_RE.replace(url, ".mp4").to_string())
                .collect()
        }

        pub(super) fn p2(value: &serde_json::Value) -> HashSet<String> {
            value
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|item| {
                    Some(
                        BODY_RE
                            .find_iter(item["content"]["rendered"].as_str()?)
                            .chain(BODY_RE.find_iter(item["excerpt"]["rendered"].as_str()?))
                            .map(|m| m.as_str().to_string())
                            .collect::<Vec<String>>(),
                    )
                })
                .flatten()
                .collect()
        }

        pub(super) fn p3(value: &serde_json::Value) -> HashSet<String> {
            value
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|item| {
                    let link = item["link"].as_str()?;
                    let date = item["date"].as_str()?;

                    let link = LINK_RE.captures(link)?;
                    let date = DATE_RE.captures(date)?;

                    let base_url = link.get(1)?.as_str();
                    let slug = link.get(2)?.as_str();
                    let year = date.get(1)?.as_str();
                    let month = date.get(2)?.as_str();

                    Some(vec![
                        format!(
                            "{}/wp-content/uploads/{}/{}/{}.mp4",
                            base_url, year, month, slug
                        ),
                        format!(
                            "{}/blog/wp-content/uploads/{}/{}/{}.mp4",
                            base_url, year, month, slug
                        ),
                    ])
                })
                .flatten()
                .collect()
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use serde_json::json;

            #[test]
            fn test_p0_retrieves_urls() {
                let json_data = json!([
                    {
                        "source_url": "http://example.com/wp-content/uploads/2003/02/image.jpg"
                    },
                    {
                        "source_url": "http://example.com/wp-content/uploads/2004/04/image.png"
                    },
                    {
                        "source_url": "https://example.com/blog/wp-content/uploads/2005/05/image.gif"
                    }
                ]);

                let urls = p0(&json_data);

                assert_eq!(urls.len(), 3);
                assert!(urls.contains("http://example.com/wp-content/uploads/2003/02/image.mp4"));
            }

            #[test]
            fn test_p0_handles_empty_string() {
                let json_data = json!([
                    {
                        "source_url": ""
                    }
                ]);

                let urls = p0(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p0_handles_empty_json() {
                let json_data = json!([{}]);

                let urls = p0(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p1_retrieves_urls() {
                let json_data = json!([
                    {
                        "_embedded":{
                            "wp:featuredmedia":[
                                {
                                    "source_url":"http://example.com/wp-content/uploads/2024/03/image1.jpg"
                                },
                                {
                                    "source_url":"http://example.com/wp-content/uploads/2024/03/image2.jpg"
                                }
                            ]
                        }
                    },
                    {
                        "_embedded":{
                            "wp:featuredmedia":[
                                {
                                    "source_url":"http://example.com/wp-content/uploads/2024/03/image2.jpg"
                                },
                                {
                                    "source_url":"http://example.com/wp-content/uploads/2024/03/image3.jpg"
                                }
                            ]
                        }
                    },
                    {
                        "_embedded":{
                            "wp:featuredmedia":[
                                {
                                    "source_url":"http://example.com/wp-content/uploads/2020/05/image2.jpg"
                                },
                                {
                                    "source_url":"http://example.com/wp-content/uploads/2021/05/video.mp4"
                                }
                            ]
                        }
                    }
                ]);

                let urls = p1(&json_data);

                assert_eq!(urls.len(), 5);
                assert!(urls.contains("http://example.com/wp-content/uploads/2020/05/image2.mp4"));
                assert!(urls.contains("http://example.com/wp-content/uploads/2021/05/video.mp4"));
            }

            #[test]
            fn test_p1_handles_empty_array() {
                let json_data = json!([
                    {
                        "_embedded": {
                            "wp:featuredmedia": []
                        }
                    }
                ]);

                let urls = p1(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p1_handles_empty_string() {
                let json_data = json!([
                    {
                        "_embedded": {
                            "wp:featuredmedia": [
                                {
                                    "source_url": ""
                                }
                            ]
                        }
                    }
                ]);

                let urls = p1(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p1_handles_empty_json() {
                let json_data = json!([
                    {
                        "_embedded": {
                            "wp:featuredmedia": [
                                {

                                }
                            ]
                        }
                    }
                ]);

                let urls = p1(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p2_retrieves_urls() {
                let json_data = json!([
                    {
                        "content": {
                            "rendered": "Check this link https://example.com/wp-content/uploads/2024/03/first-post.mp4."
                        },
                        "excerpt": {
                            "rendered": "Check this link https://example.com/wp-content/uploads/2024/03/first-post.mp4."
                        }
                    },
                    {
                        "content": {
                            "rendered": "Check this link https://example.com/wp-content/uploads/2024/03/second-post.mp4."
                        },
                        "excerpt": {
                            "rendered": "Check this link https://example.com/blog/wp-content/uploads/2024/03/third-post.mp4.
                                         And then there is also this: https://example.com/blog/wp-content/uploads/2024/03/fourth-post.mp4."
                        }
                    },
                    {
                        "content": {
                            "rendered": ""
                        },
                        "excerpt": {
                            "rendered": "Check this link https://example.com/blog/wp-content/uploads/2024/03/fifth-post.mp4"
                        }
                    }
                ]);

                let urls = p2(&json_data);

                assert_eq!(urls.len(), 5);
            }

            #[test]
            fn test_p2_handles_empty_strings() {
                let json_data = json!([
                    {
                        "content": {
                            "rendered": ""
                        },
                        "excerpt": {
                            "rendered": ""
                        }
                    }
                ]);

                let urls = p2(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p2_handles_empty_json() {
                let json_data = json!([{}]);

                let urls = p2(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p3_retrieves_urls() {
                let json_data = json!([
                    {
                        "date": "2024-03-21T18:25:20",
                        "link": "http://example.com/first-post/"
                    },
                    {
                        "date": "2015-12-15T18:25:20",
                        "link": "https://example.com/second-post/"
                    },
                    {
                        "date": "2012-01-15T18:25:20",
                        "link": "https://example.com/third-post/"
                    }
                ]);

                let urls = p3(&json_data);

                assert_eq!(urls.len(), 6);
                assert!(
                    urls.contains("https://example.com/wp-content/uploads/2015/12/second-post.mp4")
                );
                assert!(urls.contains(
                    "https://example.com/blog/wp-content/uploads/2015/12/second-post.mp4"
                ));
            }

            #[test]
            fn test_p3_handles_empty_strings() {
                let json_data = json!([
                    {
                        "date": "",
                        "link": ""
                    },
                    {
                        "date": "2024-03-21T18:25:20",
                        "link": ""
                    },
                    {
                        "date": "",
                        "link": "http://example.com/blog/first-post/"
                    }
                ]);

                let urls = p3(&json_data);

                assert!(urls.is_empty());
            }

            #[test]
            fn test_p3_handles_empty_json() {
                let json_data = json!([{}]);

                let urls = p3(&json_data);

                assert!(urls.is_empty());
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use mockito::Server;
        use serde_json::json;
        use std::vec;

        #[test]
        fn test_build_with_target_posts() {
            let config = Config {
                url: String::from("https://domain.tld"),
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

            let url = build(&config);

            assert_eq!(
                url,
                "https://domain.tld/wp-json/wp/v2/posts?per_page=100&before=2001-01-01T00:00:00&modified_before=2002-02-02T00:00:00&after=2003-03-03T00:00:00&modified_after=2004-04-04T00:00:00&exclude=5,6&_embed=wp:featuredmedia&categories_exclude=1,2&tags_exclude=3,4"
            );
        }

        #[test]
        fn test_build_with_target_media() {
            let config = Config {
                url: String::from("https://domain.tld"),
                target: Target::Media,
                before: Some(String::from("2001-01-01T00:00:00")),
                modified_before: Some(String::from("2002-02-02T00:00:00")),
                after: Some(String::from("2003-03-03T00:00:00")),
                modified_after: Some(String::from("2004-04-04T00:00:00")),
                exclude: vec![5, 6],
            };

            let url = build(&config);

            assert_eq!(
                url,
                "https://domain.tld/wp-json/wp/v2/media?per_page=100&before=2001-01-01T00:00:00&modified_before=2002-02-02T00:00:00&after=2003-03-03T00:00:00&modified_after=2004-04-04T00:00:00&exclude=5,6"
            );
        }

        #[test]
        fn test_build_with_minimal_config() {
            let config = Config {
                url: String::from("https://domain.tld"),
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

            let url = build(&config);

            assert_eq!(
                url,
                "https://domain.tld/wp-json/wp/v2/posts?per_page=100&_embed=wp:featuredmedia"
            );
        }

        #[tokio::test]
        async fn test_filter_all_valid_urls() {
            let mut server = Server::new_async().await;

            let m1 = server.mock("HEAD", "/video1.mp4").create_async().await;

            let m2 = server
                .mock("HEAD", "/video2.mp4")
                .with_status(301)
                .create_async()
                .await;

            let base_url = server.url();

            let urls: HashSet<String> = vec![
                format!("{}/video1.mp4", base_url),
                format!("{}/video2.mp4", base_url),
            ]
            .into_iter()
            .collect();

            let client = reqwest::Client::new();

            let urls = filter(&client, &urls).await.unwrap();

            m1.assert_async().await;
            m2.assert_async().await;

            assert_eq!(urls.len(), 2);
        }

        #[tokio::test]
        async fn test_filter_some_invalid_urls() {
            let mut server = Server::new_async().await;

            let m1 = server.mock("HEAD", "/valid_url").create_async().await;

            let m2 = server
                .mock("HEAD", "/invalid_url")
                .with_status(404)
                .create_async()
                .await;

            let base_url = server.url();

            let urls: HashSet<String> = [
                format!("{}/valid_url", base_url),
                format!("{}/invalid_url", base_url),
            ]
            .into_iter()
            .collect();

            let client = reqwest::Client::new();

            let url = filter(&client, &urls).await.unwrap();

            m1.assert_async().await;
            m2.assert_async().await;

            assert_eq!(url.len(), 1);
            assert!(url.contains(&format!("{}/valid_url", base_url)));
            assert!(!url.contains(&format!("{}/invalid_url", base_url)));
        }

        #[tokio::test]
        async fn test_filter_empty_set() {
            let client = reqwest::Client::new();

            let urls: HashSet<String> = HashSet::new();

            let urls = filter(&client, &urls).await.unwrap();

            assert_eq!(urls.len(), 0);
        }

        #[tokio::test]
        async fn test_filter_with_network_error() {
            let urls: HashSet<String> = [String::from("http://fake_server/fail_connection")]
                .into_iter()
                .collect();

            let client = reqwest::Client::new();

            let result = filter(&client, &urls).await;

            assert!(result.is_err(), "Expected a network error result");
        }

        #[tokio::test]
        async fn test_find_prioritizes_p0_first() {
            let mut server = Server::new_async().await;

            let base_url = server.url();

            let json_data = json!([
            {
                "date": "2000-01-01T00:00:00",
                "link": format!("{}/blog/slug/", base_url),
                "content": {
                    "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                },
                "excerpt": {
                    "rendered": format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur : {}/blog/wp-content/uploads/2001/01/video.mp4. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", base_url)
                },
                "_embedded": {
                    "wp:featuredmedia": [
                        {
                            "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                        }
                    ]
                },
                "source_url": format!("{}/blog/wp-content/uploads/2003/03/image.jpg", base_url)
            }
            ]);

            let mock = server
                .mock("HEAD", "/blog/wp-content/uploads/2003/03/image.mp4")
                .create_async()
                .await;

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            mock.assert_async().await;

            assert_eq!(urls.len(), 1);
            assert!(urls.contains(&format!(
                "{}/blog/wp-content/uploads/2003/03/image.mp4",
                base_url
            )));
        }

        #[tokio::test]
        async fn test_find_prioritizes_p1_first() {
            let mut server = Server::new_async().await;

            let base_url = server.url();

            let json_data = json!([
            {
                "date": "2000-01-01T00:00:00",
                "link": format!("{}/blog/slug/", base_url),
                "content": {
                    "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                },
                "excerpt": {
                    "rendered": format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur : {}/blog/wp-content/uploads/2001/01/video.mp4. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", base_url)
                },
                "_embedded": {
                    "wp:featuredmedia": [
                        {
                            "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                        }
                    ]
                },
            }
            ]);

            let mock = server
                .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
                .create_async()
                .await;

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            mock.assert_async().await;

            assert_eq!(urls.len(), 1);
            assert!(urls.contains(&format!(
                "{}/blog/wp-content/uploads/2002/02/image.mp4",
                base_url
            )));
        }

        #[tokio::test]
        async fn test_find_prioritizes_p2_first() {
            let mut server = Server::new_async().await;

            let base_url = server.url();

            let json_data = json!([
                {
                    "date": "2000-01-01T00:00:00",
                    "link": format!("{}/blog/slug/", base_url),
                    "content": {
                        "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    },
                    "excerpt": {
                        "rendered": format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur : {}/blog/wp-content/uploads/2001/01/video.mp4. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", base_url)
                    }
                }
            ]);

            let mock = server
                .mock("HEAD", "/blog/wp-content/uploads/2001/01/video.mp4")
                .create_async()
                .await;

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            mock.assert_async().await;

            assert_eq!(urls.len(), 1);
            assert!(urls.contains(&format!(
                "{}/blog/wp-content/uploads/2001/01/video.mp4",
                base_url
            )));
        }

        #[tokio::test]
        async fn test_find_prioritizes_p3_first() {
            let mut server = Server::new_async().await;

            let base_url = server.url();

            let json_data = json!([
                {
                    "date": "2000-01-01T00:00:00",
                    "link": format!("{}/slug/", base_url),
                }
            ]);

            let mock = server
                .mock("HEAD", "/blog/wp-content/uploads/2000/01/slug.mp4")
                .create_async()
                .await;

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            mock.assert_async().await;

            assert_eq!(urls.len(), 1);
            assert!(urls.contains(&format!(
                "{}/blog/wp-content/uploads/2000/01/slug.mp4",
                base_url
            )));
        }

        #[tokio::test]
        async fn test_find_fallbacks_when_higher_priority_fails() {
            let mut server = Server::new_async().await;

            let base_url = server.url();

            let json_data = json!([
                {
                    "date": "2000-01-01T00:00:00",
                    "link": format!("{}/blog/slug/", base_url),
                    "content": {
                        "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    },
                    "excerpt": {
                        "rendered": format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur : {}/blog/wp-content/uploads/2001/01/video.mp4. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", base_url)
                    },
                    "_embedded": {
                        "wp:featuredmedia": [
                            {
                                "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                            }
                        ]
                    },
                    "source_url": ""
                }
            ]);

            let m1 = server
                .mock("HEAD", "/blog/wp-content/uploads/2001/01/video.mp4")
                .with_status(301)
                .create_async()
                .await;

            let m2 = server
                .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
                .with_status(404)
                .create_async()
                .await;

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            m1.assert_async().await;
            m2.assert_async().await;

            assert_eq!(urls.len(), 1);
            assert!(urls.contains(&format!(
                "{}/blog/wp-content/uploads/2001/01/video.mp4",
                base_url
            )));
        }

        #[tokio::test]
        async fn test_find_returns_empty_hash_set_when_all_fail() {
            let mut server = Server::new_async().await;

            let base_url = server.url();

            let json_data = json!([
                {
                    "date": "",
                    "link": "",
                    "content": {
                        "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    },
                    "excerpt": {
                        "rendered": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    },
                    "_embedded": {
                        "wp:featuredmedia": [
                            {
                                "source_url": format!("{}/blog/wp-content/uploads/2002/02/image.jpg", base_url)
                            },
                            {
                                "source_url": format!("{}/blog/wp-content/uploads/2003/03/image.jpg", base_url)
                            }
                        ]
                    },
                    "source_url": format!("{}/blog/wp-content/uploads/2004/04/image.jpg", base_url)
                }
            ]);

            let m1 = server
                .mock("HEAD", "/blog/wp-content/uploads/2002/02/image.mp4")
                .with_status(403)
                .create_async()
                .await;

            let m2 = server
                .mock("HEAD", "/blog/wp-content/uploads/2003/03/image.mp4")
                .with_status(404)
                .create_async()
                .await;

            let m3 = server
                .mock("HEAD", "/blog/wp-content/uploads/2004/04/image.mp4")
                .with_status(500)
                .create_async()
                .await;

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            m1.assert_async().await;
            m2.assert_async().await;
            m3.assert_async().await;

            assert!(urls.is_empty());
        }

        #[tokio::test]
        async fn test_find_handles_empty_json() {
            let json_data = json!([{}]);

            let client = reqwest::Client::new();

            let urls = find(&client, &json_data).await.unwrap();

            assert!(urls.is_empty());
        }

        #[tokio::test]
        async fn test_find_with_network_error() {
            let json_data = json!([
                {
                    "source_url": "http://fake_server/blog/wp-content/uploads/2001/01/image.jpg"
                }
            ]);

            let client = reqwest::Client::new();

            let result = find(&client, &json_data).await;

            assert!(result.is_err(), "Expected a network error result");
        }
    }
}
