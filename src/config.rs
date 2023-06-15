use std::path::PathBuf;

use confique::Config as Confique;

#[derive(Confique)]
pub struct Config {
    source_directory: PathBuf,
    target_directory: PathBuf,
}
