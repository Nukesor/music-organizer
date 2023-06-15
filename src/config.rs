use std::path::{Path, PathBuf};

use confique::Config as Confique;
use shellexpand::tilde;

#[derive(Confique)]
pub struct Config {
    source_directory: PathBuf,
    target_directory: PathBuf,
}

/// Little helper which expands a given path's `~` characters to a fully qualified path.
pub fn expand_home(old_path: &Path) -> PathBuf {
    PathBuf::from(tilde(&old_path.to_string_lossy()).into_owned())
}

impl Config {
    pub fn source_directory(&self) -> PathBuf {
        expand_home(&self.source_directory)
    }

    pub fn target_directory(&self) -> PathBuf {
        expand_home(&self.target_directory)
    }
}
