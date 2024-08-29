use clap::{Parser, ValueEnum};

/// Searches for mp4 videos for a given WordPress website.
#[derive(Parser)]
#[command(name = "pf", author, version, about)]
pub struct Opt {
    /// WordPress base URL (e.g. http://example.com).
    pub url: String,

    /// Searching mode.
    #[arg(value_enum)]
    pub mode: Mode,

    /// Result set published before a given date (cf. https://core.trac.wordpress.org/ticket/41032).
    #[arg(long)]
    pub before: Option<String>,

    /// Result set modified before a given date (cf. https://core.trac.wordpress.org/ticket/41032).
    #[arg(long)]
    pub modified_before: Option<String>,

    /// Result set published after a given date (cf. https://core.trac.wordpress.org/ticket/41032).
    #[arg(long)]
    pub after: Option<String>,

    /// Result set modified after a given date (cf. https://core.trac.wordpress.org/ticket/41032).
    #[arg(long)]
    pub modified_after: Option<String>,

    /// Ensures result set excludes specific IDs.
    #[arg(short, long)]
    pub exclude: Vec<u16>,

    /// Ensures result set excludes specific category IDs.
    #[arg(long)]
    pub categories_exclude: Vec<u16>,

    /// Ensures result set excludes to specific tag IDs.
    #[arg(long)]
    pub tags_exclude: Vec<u16>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Fetch from posts only.
    Fast,
    /// Fetch from both posts and media.
    Slow,
}
