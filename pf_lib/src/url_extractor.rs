use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
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

/// Extracts URLs from the `source_url` field of media items.
///
/// This function filters the `source_url` field of media items, matches them against the `SRC_RE`
/// regex, and replaces the file extension with `.mp4`.
///
/// # Arguments
///
/// * `values` - A slice of `serde_json::Value` representing media items.
///
/// # Returns
///
/// A `HashSet` containing the extracted URLs.
pub fn p0(values: &[Value]) -> HashSet<String> {
    values
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
/// # Arguments
///
/// * `value` - A slice of `serde_json::Value` representing posts.
///
/// # Returns
///
/// A `HashSet` containing the extracted URLs.
pub fn p1(value: &[Value]) -> HashSet<String> {
    value
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
/// # Arguments
///
/// * `value` - A slice of `serde_json::Value` representing posts.
///
/// # Returns
///
/// A `HashSet` containing the extracted URLs.
pub fn p2(value: &[Value]) -> HashSet<String> {
    value
        .iter()
        .flat_map(|item| {
            Some(
                BODY_RE
                    .find_iter(item["content"]["rendered"].as_str()?)
                    .chain(
                        BODY_RE.find_iter(
                            item["excerpt"]["rendered"]
                                .as_str()?
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
/// # Arguments
///
/// * `value` - A slice of `serde_json::Value` representing posts.
///
/// # Returns
///
/// A `HashSet` containing the constructed URLs.
pub fn p3(value: &[Value]) -> HashSet<String> {
    value
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_p0() {
        let values = vec![
            json!({"source_url": "http://example.com/wp-content/uploads/2021/01/video.mp4"}),
            json!({"source_url": "http://example.com/wp-content/uploads/2021/01/image.jpg"}),
            json!({"source_url": "http://example.com/blog/wp-content/uploads/2021/01/video.mp4"}),
        ];
        let result = p0(&values);
        let expected: HashSet<String> = vec![
            "http://example.com/wp-content/uploads/2021/01/video.mp4".to_string(),
            "http://example.com/wp-content/uploads/2021/01/image.mp4".to_string(),
            "http://example.com/blog/wp-content/uploads/2021/01/video.mp4".to_string(),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_p1() {
        let values = vec![
            json!({"_embedded": {"wp:featuredmedia": [{"source_url": "http://example.com/wp-content/uploads/2021/01/video.mp4"}]}}),
            json!({"_embedded": {"wp:featuredmedia": [{"source_url": "http://example.com/wp-content/uploads/2021/01/image.jpg"}]}}),
            json!({"_embedded": {"wp:featuredmedia": [{"source_url": "http://example.com/blog/wp-content/uploads/2021/01/video.mp4"}]}}),
        ];
        let result = p1(&values);
        let expected: HashSet<String> = vec![
            "http://example.com/wp-content/uploads/2021/01/video.mp4".to_string(),
            "http://example.com/wp-content/uploads/2021/01/image.mp4".to_string(),
            "http://example.com/blog/wp-content/uploads/2021/01/video.mp4".to_string(),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_p2() {
        let values = vec![
            json!({"content": {"rendered": "http://example.com/wp-content/uploads/2021/01/video-1.mp4"}, "excerpt": {"rendered": ""}}),
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "http://example.com/wp-content/uploads/2021/01/video-2.mp4"}}),
            json!({"content": {"rendered": "http://example.com/blog/wp-content/uploads/2021/01/video-3.mp4"}, "excerpt": {"rendered": ""}}),
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video-4.mov"}}),
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video-5.MOV"}}),
            json!({"content": {"rendered": ""}, "excerpt": {"rendered": "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video-6.MP4"}}),
        ];
        let result = p2(&values);
        let expected: HashSet<String> = vec![
            "http://example.com/wp-content/uploads/2021/01/video-1.mp4".to_string(),
            "http://example.com/wp-content/uploads/2021/01/video-2.mp4".to_string(),
            "http://example.com/blog/wp-content/uploads/2021/01/video-3.mp4".to_string(),
            "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video-4.mov"
                .to_string(),
            "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video-5.mov"
                .to_string(),
            "https:\\/\\/www.example.com\\/wp-content\\/uploads\\/2021\\/01\\/video-6.mp4"
                .to_string(),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_p3() {
        let values = vec![
            json!({"link": "http://example.com/post-slug", "date": "2021-01-01T00:00:00"}),
            json!({"link": "http://example.com/blog/post-slug", "date": "2021-01-01T00:00:00"}),
        ];
        let result = p3(&values);
        let expected: HashSet<String> = vec![
            "http://example.com/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
            "http://example.com/blog/wp-content/uploads/2021/01/post-slug.mp4".to_string(),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
}
