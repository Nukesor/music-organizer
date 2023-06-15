use std::{
    collections::{HashMap, VecDeque},
    fs::DirEntry,
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Context, Result};

use crate::models::release::*;

/// Check a directory of artists.
/// First up, we get the directories of the first level, which are the artists.
/// Next we get the directories for each artist, which are the releases.
pub fn check_directory(path: PathBuf) -> Result<()> {
    let artists = read_dir_or_fail(&path).context("Failed to get list of artist directories")?;

    let mut artist_releases: HashMap<String, Vec<Release>> = HashMap::new();
    for artist in artists {
        let release_dirs = read_dir_or_fail(&artist.path()).context(format!(
            "Failed to get list of releases for artist: {artist:?}"
        ))?;

        // Get all releases for this artist.
        let mut releases = Vec::new();
        for release_dir in release_dirs {
            // Ignore failed release gets, there'll be a error message for each failure.
            let _ = get_release_name(&mut releases, &release_dir.path());
        }

        // Add all releases for that artist to the final hashmap.
        let artist_name = artist.file_name().to_string_lossy().to_string();
        artist_releases.insert(artist_name, releases);
    }

    for (artist, releases) in artist_releases {
        println!("\n\nReleases for Artist '{artist}':");
        for release in releases {}
    }

    Ok(())
}

/// Check, if the name of a release matches the expected format.
pub fn get_release_name(releases: &mut Vec<Release>, path: &PathBuf) -> Result<()> {
    let name = path.file_name().context(format!(
        "Failed to get filename of directory at path {path:?}"
    ))?;

    // Split the name of the release into its respective parts.
    let mut parts: VecDeque<String> = name
        .to_string_lossy()
        .split(" - ")
        .map(|name| name.to_string())
        .collect();

    // Check if the release name has the expected amount of parts
    if parts.len() != 6 {
        println!("Release '{name:?}' has {} instead of 6 parts", parts.len());
    }

    let artist = parts.pop_front().unwrap();
    let release_name = parts.pop_front().unwrap();
    let release_id = parts.pop_front().unwrap();
    let year = parts.pop_front().unwrap();
    let format = parts.pop_front().unwrap();
    let source = parts.pop_front().unwrap();

    let year: usize = match year.parse() {
        Ok(year) => year,
        Err(_) => {
            println!("Failed to parse {year} as a year on release {name:?}");
            return Ok(());
        }
    };

    let release = Release {
        artist,
        release_name,
        release_id,
        year,
        format: Format::from_str(&format)
            .context("Failed to parse format {format} for release {name:?}")?,
        source: Source::from_str(&source)
            .context("Failed to parse source {source} for release {name:?}")?,
    };
    releases.push(release);

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
