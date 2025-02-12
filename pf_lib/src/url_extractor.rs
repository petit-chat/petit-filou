use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

use crate::mime_types::SUPPORTED_MIME_TYPES;

lazy_static! {
    /// Regular expression to match source URLs of media files.
    static ref SRC_RE: Regex =
        Regex::new(r"^https?://[^/]+(?:/blog)?/wp-content/uploads/\d{4}/\d{2}/[^/]+\.\w+$")
            .unwrap();

    /// Regular expression to match file extensions.
    static ref EXT_RE: Regex = Regex::new(r"\.\w+$").unwrap();

    /// Regular expression to match URLs of media files in the content body.
    static ref BODY_RE: Regex = {
        let mut pattern = r"https?:(?:\\)?/(?:\\)?/[^/]+(?:/blog)?(?:\\)?/wp-content(?:\\)?/uploads(?:\\)?/\d{4}(?:\\)?/\d{2}(?:\\)?/[^/]+\.(?:".to_string();

        pattern.push_str(
            &SUPPORTED_MIME_TYPES
                .iter()
                .map(|(ext, _)| *ext)
                .collect::<Vec<_>>()
                .join("|"),
        );

        pattern.push(')');

        Regex::new(&pattern).unwrap()
    };

    /// Regular expression to match and capture base URL and slug of a complex URL.
    static ref LINK_RE: Regex = Regex::new(r"^(https?://[^/]+)(?:/[^/]+)*/([^/]+)/?$").unwrap();

    /// Regular expression to match and capture month and year of a date string.
    static ref DATE_RE: Regex = Regex::new(r"^(\d{4})-(\d{2})-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap();
}

pub struct Xtract {
    json: Vec<serde_json::Value>,
}

impl Xtract {
    pub fn new(json: &str) -> Self {
        Self {
            json: serde_json::from_str::<Vec<serde_json::Value>>(json).unwrap(),
        }
    }

    /// Extracts URLs from the `source_url` field of media items.
    ///
    /// This function filters the `source_url` field of media items, matches them against the `SRC_RE`
    /// regex, and replaces the file extension with `.mp4`.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the extracted URLs.
    fn p0(&self) -> HashSet<String> {
        self.json
            .iter()
            .filter_map(|media| media["source_url"].as_str())
            .filter(|&url| SRC_RE.is_match(url))
            .map(|url| EXT_RE.replace(url, ".mp4").to_string())
            .collect()
    }

    /// Extracts URLs from the `source_url` field of featured media in posts.
    ///
    /// This function filters the `source_url` field of featured media in posts, matches them against
    /// the `SRC_RE` regex, and replaces the file extension with `.mp4`.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the extracted URLs.
    fn p1(&self) -> HashSet<String> {
        self.json
            .iter()
            .filter_map(|post| post["_embedded"]["wp:featuredmedia"].as_array())
            .flatten()
            .filter_map(|media| media["source_url"].as_str())
            .filter(|&url| SRC_RE.is_match(url))
            .map(|url| EXT_RE.replace(url, ".mp4").to_string())
            .collect()
    }

    /// Extracts URLs from the content and excerpt fields of posts.
    ///
    /// This function searches for URLs in the `content.rendered` and `excerpt.rendered` fields of posts,
    /// matches them against the `BODY_RE` regex, and collects them.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the extracted URLs.
    fn p2(&self) -> HashSet<String> {
        self.json
            .iter()
            .flat_map(|item| {
                Some(
                    BODY_RE
                        .find_iter(item["content"]["rendered"].as_str()?)
                        .chain(
                            BODY_RE.find_iter(
                                item["excerpt"]["rendered"]
                                    .as_str()?
                                    .replace('\\', "")
                                    .to_lowercase()
                                    .as_str(),
                            ),
                        )
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<String>>(),
                )
            })
            .flatten()
            .collect()
    }

    /// Constructs URLs based on the `link` and `date` fields of posts.
    ///
    /// This function constructs URLs for media files based on the `link` and `date` fields of posts,
    /// using the `LINK_RE` and `DATE_RE` regexes to extract parts of the URL and date.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the constructed URLs.
    fn p3(&self) -> HashSet<String> {
        self.json
            .iter()
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

    /// Extracts URLs from JSON string.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the extracted URLs.
    pub fn run(&self) -> HashSet<String> {
        self.p0()
            .into_iter()
            .chain(self.p1())
            .chain(self.p2())
            .chain(self.p3())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_new() {
        Xtract::new("[]");
    }

    #[test]
    #[should_panic]
    fn test_new_with_invalid_json() {
        Xtract::new("invalid json");
    }

    #[test]
    #[should_panic]
    fn test_new_with_non_wordpress_json() {
        Xtract::new("{}");
    }

    #[test]
    fn test_p0() {
        assert_eq!(Xtract {
            json: vec![
                json!({"source_url": "http://example.com/wp-content/uploads/2021/01/video.mp4"}),
            ]
        }
        .p0().iter().next().unwrap(),
        "http://example.com/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p0_with_jpg() {
        assert_eq!(Xtract {
            json: vec![
                json!({"source_url": "http://example.com/wp-content/uploads/2021/01/image.jpg"}),
            ]
        }
        .p0().iter().next().unwrap(),
        "http://example.com/wp-content/uploads/2021/01/image.mp4")
    }

    #[test]
    fn test_p0_with_blog_prefix() {
        assert_eq!(Xtract { json: vec![
            json!({"source_url": "http://example.com/blog/wp-content/uploads/2021/01/video.mp4"}),
        ] }.p0().iter().next().unwrap(), "http://example.com/blog/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p1() {
        assert_eq!(Xtract { json: vec![
            json!({"_embedded": {"wp:featuredmedia": [{"source_url": "http://example.com/wp-content/uploads/2021/01/video.mp4"}]}}),
        ] }.p1().iter().next().unwrap(), "http://example.com/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p1_with_jpg() {
        assert_eq!(Xtract { json: vec![
            json!({"_embedded": {"wp:featuredmedia": [{"source_url": "http://www.example.com/wp-content/uploads/2021/01/image.jpg"}]}}),
        ] }.p1().iter().next().unwrap(), "http://www.example.com/wp-content/uploads/2021/01/image.mp4")
    }

    #[test]
    fn test_p1_with_blog_prefix() {
        assert_eq!(Xtract { json: vec![
            json!({"_embedded": {"wp:featuredmedia": [{"source_url": "http://example.com/blog/wp-content/uploads/2021/01/video.mp4"}]}}),
        ] }.p1().iter().next().unwrap(), "http://example.com/blog/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p2_with_content_field() {
        assert_eq!(Xtract { json: vec![
            json!({"content": {"rendered": "http://example.com/wp-content/uploads/2021/01/video.mp4"}, "excerpt": {"rendered": ""}}),
        ] }.p2().iter().next().unwrap(), "http://example.com/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p2_with_excerpt_field() {
        assert_eq!(Xtract { json: vec![
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https://example.com/wp-content/uploads/2021/01/video.mp4"}}),
        ] }.p2().iter().next().unwrap(), "https://example.com/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p2_with_blog_prefix() {
        assert_eq!(Xtract { json: vec![
            json!({"content": {"rendered": "https://www.example.com/blog/wp-content/uploads/2021/01/video.mp4"}, "excerpt": {"rendered": ""}}),
        ] }.p2().iter().next().unwrap(), "https://www.example.com/blog/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p2_with_backslashes_and_mov() {
        assert_eq!(Xtract { json: vec![
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video.mov"}}),
        ] }.p2().iter().next().unwrap(), "https://www.example.com/wp-content/uploads/2021/01/video.mov")
    }

    #[test]
    fn test_p2_with_backslashes_and_caps_mov() {
        assert_eq!(Xtract { json: vec![
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video.MOV"}}),
        ] }.p2().iter().next().unwrap(), "https://www.example.com/wp-content/uploads/2021/01/video.mov")
    }

    #[test]
    fn test_p2_with_backslashes_and_caps_mp4() {
        assert_eq!(Xtract { json: vec![
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video.MP4"}}),
        ] }.p2().iter().next().unwrap(), "https://www.example.com/wp-content/uploads/2021/01/video.mp4")
    }

    #[test]
    fn test_p3() {
        assert_eq!(
            Xtract {
                json: vec![
                    json!({"link": "http://example.com/post-slug", "date": "2021-01-01T00:00:00"}),
                ]
            }
            .p3(),
            vec![
                "http://example.com/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
                "http://example.com/blog/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
            ]
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn test_p3_with_blog_prefix() {
        assert_eq!(Xtract {
            json: vec![
                json!({"link": "http://example.com/blog/post-slug", "date": "2021-01-01T00:00:00"}),
            ]
        }
        .p3(), vec![
            "http://example.com/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
            "http://example.com/blog/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
        ]
        .into_iter()
        .collect()
        )
    }

    #[test]
    fn test_run() {
        assert_eq!(
            Xtract::new(r#"[
            {"source_url": "http://example.com/wp-content/uploads/2021/01/video-1.mp4"},
            {"_embedded": {"wp:featuredmedia": [{"source_url": "http://example.com/wp-content/uploads/2021/01/video-2.mp4"}]}},
            {"content": {"rendered": "http://example.com/wp-content/uploads/2021/01/video-3.mp4"}, "excerpt": {"rendered": ""}},
            {"link": "http://example.com/post-slug", "date": "2021-01-01T00:00:00"}
        ]"#).run(), vec![
            "http://example.com/wp-content/uploads/2021/01/video-1.mp4".to_string(),
            "http://example.com/wp-content/uploads/2021/01/video-2.mp4".to_string(),
            "http://example.com/wp-content/uploads/2021/01/video-3.mp4".to_string(),
            "http://example.com/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
            "http://example.com/blog/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
        ]
        .into_iter()
        .collect()
    )
    }
}
