use std::{fs::DirEntry, path::PathBuf};

use anyhow::{Context, Result};

/// Check a directory of artists.
/// First up, we get the directories of the first level, which are the artists.
/// Next we get the directories for each artist, which are the releases.
pub fn check_directory(path: PathBuf) -> Result<()> {
    let artists = read_dir_or_fail(&path).context("Failed to get list of artist directories")?;

    for artist in artists {
        let releases = read_dir_or_fail(&artist.path()).context(format!(
            "Failed to get list of releases for artist: {artist:?}"
        ))?;

        for release in releases {
            check_release_name(&release.path())?;
        }
    }

    Ok(())
}

/// Check, if the name of a releaes matches the expected format.
pub fn check_release_name(path: &PathBuf) -> Result<()> {
    let name = path.file_name().context(format!(
        "Failed to get filename of directory at path {path:?}"
    ))?;

    // Split the name of the release into its respective parts.
    let parts: Vec<String> = name
        .to_string_lossy()
        .split(" - ")
        .map(|name| name.to_string())
        .collect();

    // Check if the release name has the expected amount of parts
    if parts.len() != 6 {
        println!("Release {name:?} has {} instead of 6 parts", parts.len());
    }

    let Some(artist) = parts.remove(0) else {
        return Ok(());
    };

    let Some(release_name) = parts.remove(0) else {
        return Ok(());
    };

    let Some(release_id) = parts.remove(0) else {
        return Ok(());
    };

    let Some(year) = parts.remove(0) else {
        return Ok(());
    };
    let year: u32 = match year.strip().parse() {
        Ok(year) => year,
        Err(error) => {
            println!("Failed to parse {year} as a year on release {name:?}");
            return Ok(());
        }
    };

    Ok(())
}

/// Read all entries of a directory and return them.
/// If a FileType is specified, only files with that type will be returned.
pub fn read_dir_or_fail(path: &PathBuf) -> Result<Vec<DirEntry>> {
    let dir = std::fs::read_dir(path)?;

    let mut entries: Vec<DirEntry> = Vec::new();
    for entry_result in dir {
        let entry = entry_result?;

        // Filter everything that's not a directory
        if !entry.file_type()?.is_dir() {
            continue;
        }

        entries.push(entry);
    }

    Ok(entries)
}
