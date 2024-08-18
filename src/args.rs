use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Wordpress website base URL (e.g. https://domain.tld).
    pub url: String,

    /// Searching mode.
    #[arg(value_enum)]
    pub mode: Mode,

    /// Result set published before a given ISO8601 compliant date.
    #[arg(long)]
    pub before: Option<String>,

    /// Result set modified before a given ISO8601 compliant date.
    #[arg(long)]
    pub modified_before: Option<String>,

    /// Result set published after a given ISO8601 compliant date.
    #[arg(long)]
    pub after: Option<String>,

    /// Result set modified after a given ISO8601 compliant date.
    #[arg(long)]
    pub modified_after: Option<String>,

    /// Ensures result set excludes specific IDs.
    #[arg(short, long)]
    pub exclude: Vec<u16>,

    /// Ensures result set excludes specific categorie IDs.
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
