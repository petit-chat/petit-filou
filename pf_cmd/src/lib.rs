use clap::Parser;
use futures_util::pin_mut;
use futures_util::StreamExt;

/// Scans WordPress websites to find videos.
///
/// Supported MIME types: video/mp4 and video/quicktime (.mov).
#[derive(Parser)]
#[command(name = "pf", author, version, about)]
pub struct Opt {
    /// WordPress base URL (e.g. <http://example.com>).
    pub url: String,

    /// Result set published before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long)]
    pub before: Option<String>,

    /// Result set modified before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long)]
    pub modified_before: Option<String>,

    /// Result set published after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long)]
    pub after: Option<String>,

    /// Result set modified after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long)]
    pub modified_after: Option<String>,

    /// Ensures result set excludes specific IDs.
    #[arg(long)]
    pub exclude: Vec<u16>,

    /// Ensures result set excludes specific category IDs.
    #[arg(long)]
    pub categories_exclude: Vec<u16>,

    /// Ensures result set excludes to specific tag IDs.
    #[arg(long)]
    pub tags_exclude: Vec<u16>,
}

impl Opt {
    /// Converts the `Opt` struct to a `FinderConfig` struct.
    fn to_finder_config(&self) -> pf_lib::FinderConfig {
        pf_lib::FinderConfig {
            url: self.url.clone(),
            target: pf_lib::FinderTarget::Posts {
                categories_exclude: self.categories_exclude.clone(),
                tags_exclude: self.tags_exclude.clone(),
            },
            before: self.before.clone(),
            modified_before: self.modified_before.clone(),
            after: self.after.clone(),
            modified_after: self.modified_after.clone(),
            exclude: self.exclude.clone(),
        }
    }
}

/// Runs the `pf` command.
pub async fn run(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = opt.to_finder_config();
    print_stream(&config).await?;
    config.target = pf_lib::FinderTarget::Media;
    print_stream(&config).await?;
    Ok(())
}

/// Consumes and prints the `find` stream.
async fn print_stream(config: &pf_lib::FinderConfig) -> Result<(), Box<dyn std::error::Error>> {
    let stream = pf_lib::find(config);
    pin_mut!(stream);
    while let Some(res) = stream.next().await {
        match res {
            Ok(url) => println!("{}", url),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
