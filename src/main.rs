use clap::Parser;
use petit_filou::args::{Cli, Mode};
use petit_filou::{Config, Target};
use std::collections::HashSet;
use std::error::Error;

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
