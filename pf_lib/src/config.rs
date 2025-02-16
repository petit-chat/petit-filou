/// Represents the target type for the `Finder`.
///
/// This enum is used to specify whether the `Finder` should target media or posts.
/// The `Posts` variant includes fields to exclude specific categories and tags.
#[derive(Default, PartialEq, Debug)]
pub enum FinderTarget {
    /// Target media items.
    #[default]
    Media,
    /// Target posts with options to exclude specific categories and tags.
    Posts {
        /// Categories to exclude.
        categories_exclude: Vec<u16>,
        /// Tags to exclude.
        tags_exclude: Vec<u16>,
    },
}

/// Configuration for the `Finder`.
///
/// This struct holds various configuration options for the `Finder`, including the WordPress base URL,
/// the target type, date filters, and items to exclude.
#[derive(Default)]
pub struct FinderConfig {
    /// The WordPress base URL.
    pub url: String,

    /// The target type for the `Finder`.
    pub target: FinderTarget,

    /// Result set published before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    pub before: Option<String>,

    /// Result set modified before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    pub modified_before: Option<String>,

    /// Result set published after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    pub after: Option<String>,

    /// Result set modified after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    pub modified_after: Option<String>,

    /// Ensures result set excludes specific IDs.
    pub exclude: Vec<u16>,
}
