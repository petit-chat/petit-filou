use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use petit_filou::args::Cli;
use std::fs;
use std::io::Result;

/// Shell completions can be created with:
/// `cargo run --bin generate-completions`
fn main() -> Result<()> {
    let out_dir = String::from("completions");
    fs::create_dir_all(&out_dir)?;
    let mut app = Cli::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, env!("CARGO_PKG_NAME"), &out_dir)?;
    }
    println!("Completion scripts generated under {out_dir:?}");
    Ok(())
}
