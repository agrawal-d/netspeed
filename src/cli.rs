use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = "My crate")]
pub struct Cli {
    #[arg(name = "file", help = "Path to file")]
    pub file: Option<PathBuf>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
