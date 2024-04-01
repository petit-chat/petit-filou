use clap::{Parser, ValueEnum};
use petit_filou::{Config, Target};
use std::collections::HashSet;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Wordpress website base URL (e.g. https://domain.tld).
    url: String,

    /// Searching mode.
    #[arg(value_enum)]
    mode: Mode,

    /// Result set published before a given ISO8601 compliant date.
    #[arg(long)]
    before: Option<String>,

    /// Result set modified before a given ISO8601 compliant date.
    #[arg(long)]
    modified_before: Option<String>,

    /// Result set published after a given ISO8601 compliant date.
    #[arg(long)]
    after: Option<String>,

    /// Result set modified after a given ISO8601 compliant date.
    #[arg(long)]
    modified_after: Option<String>,

    /// Ensures result set excludes specific IDs.
    #[arg(short, long)]
    exclude: Vec<u16>,

    /// Ensures result set excludes specific categorie IDs.
    #[arg(long)]
    categories_exclude: Vec<u16>,

    /// Ensures result set excludes to specific tag IDs.
    #[arg(long)]
    tags_exclude: Vec<u16>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Fetch from posts only.
    Fast,
    /// Fetch from both posts and media.
    Slow,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut urls = HashSet::new();

    let target = Target::Posts {
        categories_exclude: cli.categories_exclude.clone(),
        tags_exclude: cli.tags_exclude.clone(),
    };
    let ret = run_with_target(&cli, target).await?;
    urls.extend(ret);

    if cli.mode == Mode::Slow {
        let ret = run_with_target(&cli, Target::Media).await?;
        urls.extend(ret);
    }

    for url in urls {
        println!("{}", url);
    }

    Ok(())
}

async fn run_with_target(cli: &Cli, target: Target) -> Result<HashSet<String>, reqwest::Error> {
    let config = Config {
        url: cli.url.clone(),
        target,
        before: cli.before.clone(),
        modified_before: cli.modified_before.clone(),
        after: cli.after.clone(),
        modified_after: cli.modified_after.clone(),
        exclude: cli.exclude.clone(),
    };
    petit_filou::run(&config).await
}
