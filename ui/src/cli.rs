use clap::Parser;
#[derive(Parser)]
#[command(
    name = "netspeed",
    author,
    version,
    about = "Monitor network speeds and statistics",
    long_about = "Monitor all the networks in your computer in real time in a GUI. \
    Supports wireless, ethernet, loopback and all network interfaces. \
    View stats like upload, download speeds, total data transmitted, and more."
)]
pub struct Cli {}

pub fn parse_args() -> Cli {
    Cli::parse()
}
