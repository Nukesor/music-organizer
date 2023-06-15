use anyhow::Result;
use clap::Parser;

use args::CliArguments;
use confique::Config;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

mod args;
mod check;
mod config;
mod models;

fn main() -> Result<()> {
    // Read any .env files
    dotenv::dotenv().ok();
    // Parse commandline options.
    let args = CliArguments::parse();

    // Initalize everything
    init_app(args.verbose)?;

    // Read the config from the environment by default.
    // Also read from the configuration, if we find a config directory.
    let mut config_builder = config::Config::builder().env();
    if let Some(config_dir) = dirs::config_dir() {
        config_builder = config_builder.file(config_dir.join("organizer.yml"));
    }
    let config = config_builder.load()?;

    check::check_directory(config.source_directory())?;

    Ok(())
}

/// Init better_panics
/// Initialize logging
fn init_app(verbosity: u8) -> Result<()> {
    // Beautify panics for better debug output.
    better_panic::install();

    // Set the verbosity level and initialize the logger.
    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    Builder::new().filter_level(level).init();

    Ok(())
}
