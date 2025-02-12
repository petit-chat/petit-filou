/// Supported MIME types for scraping. Each tuple contains a file extension and its corresponding MIME type.
pub const SUPPORTED_MIME_TYPES: &[(&str, &str)] = &[
    #[cfg(feature = "mp4")]
    ("mp4", "video/mp4"),
    #[cfg(feature = "avi")]
    ("avi", "video/x-msvideo"),
    #[cfg(feature = "flv")]
    ("flv", "video/x-flv"),
    #[cfg(feature = "mpeg")]
    ("mpeg", "video/mpeg"),
    #[cfg(feature = "mov")]
    ("mov", "video/quicktime"),
    #[cfg(feature = "webm")]
    ("webm", "video/webm"),
    #[cfg(feature = "wmv")]
    ("wmv", "video/x-ms-wmv"),
];
