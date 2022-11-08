use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Returns relative file paths of the children of this directory
pub fn directory_children(dir: &PathBuf, show_hidden: bool) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .filter_map(|d| d.ok()) // Ignore entries with errors
        .map(|p| p.path())
        .filter(|c| !c.file_name().unwrap().to_str().unwrap().starts_with('.') || show_hidden)
        .collect())
}
