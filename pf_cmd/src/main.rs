use clap::Parser;

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = pf_cmd::Opt::parse();
    pf_cmd::run(opt).await
}
