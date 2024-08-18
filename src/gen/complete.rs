use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use petit_filou::args::Cli;
use std::env;
use std::io::Result;

/// Shell completions can be created with:
/// `cargo run --bin generate-complete`
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let mut app = Cli::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, env!("CARGO_PKG_NAME"), &out_dir)?;
    }
    println!("Completion scripts generated under {out_dir:?}");
    Ok(())
}
