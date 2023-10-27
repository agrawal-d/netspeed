use anyhow::{Context, Result};
use simplelog::*;
use std::{fs::File, path::PathBuf};

fn init_logging() -> Result<()> {
    // config with logging line and file
    let config: Config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .set_time_level(LevelFilter::Off)
        .build();

    let log_file_path = dirs::state_dir()
        .unwrap_or(PathBuf::from("/tmp"))
        .join("netspeed.log");

    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();

    if let Ok(file) = File::create(&log_file_path) {
        loggers.push(WriteLogger::new(LevelFilter::Debug, config.clone(), file));
    }
    loggers.push(TermLogger::new(
        LevelFilter::Warn,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ));

    CombinedLogger::init(loggers)?;
    println!("Logging to {:?}", log_file_path);
    info!("Logging initialized");
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    ui::cli::parse_args();
    init_logging()
        .context("Failed to initialize logging")
        .unwrap();

    ui::ui()
}
