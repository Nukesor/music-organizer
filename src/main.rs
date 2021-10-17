use anyhow::Result;
use clap::Clap;

use cli::CliArguments;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

mod cli;

fn main() -> Result<()> {
    // Parse commandline options.
    let opt = CliArguments::parse();

    // Initalize everything
    init_app(opt.verbose)?;

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