use clap::CommandFactory;
use clap_mangen::Man;
use petit_filou::args::Cli;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

/// Man page can be created with:
/// `cargo run --bin generate-man`
fn main() -> Result<()> {
    let out_dir = String::from("man");
    fs::create_dir_all(&out_dir)?;
    let out_path = PathBuf::from(out_dir).join(format!("{}.1", env!("CARGO_PKG_NAME")));
    let app = Cli::command();
    let man = Man::new(app);
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    fs::write(&out_path, buffer)?;
    println!("Man page generated under {out_path:?}");
    Ok(())
}
