use clap::Parser;
use pf_cli::{Mode, Opt};
use pf_lib::{FinderConfig, FinderTarget};
use std::error::Error;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let opt = Opt::parse();

    let mut config = FinderConfig {
        url: opt.url.clone(),
        target: FinderTarget::Posts {
            categories_exclude: opt.categories_exclude.clone(),
            tags_exclude: opt.tags_exclude.clone(),
        },
        before: opt.before.clone(),
        modified_before: opt.modified_before.clone(),
        after: opt.after.clone(),
        modified_after: opt.modified_after.clone(),
        exclude: opt.exclude.clone(),
    };

    for url in pf_lib::find(&config) {
        println!("{}", url);
    }

    if opt.mode == Mode::Fast {
        return Ok(());
    }

    config.target = FinderTarget::Media;

    for url in pf_lib::find(&config) {
        println!("{}", url);
    }

    Ok(())
}
