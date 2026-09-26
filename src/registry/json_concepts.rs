use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::debug;

use crate::concept::Concept;

pub fn get_files() -> color_eyre::Result<Vec<PathBuf>> {
    let concepts_dir = Path::new("./concepts");

    let entries = match fs::read_dir(concepts_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            debug!("Skipping loading concepts: The concepts directory was not found.");
            return Ok(Vec::new());
        }
        Err(_) => {
            debug!("Skipping loading concepts: The concepts directory could not be read.");
            return Ok(Vec::new());
        }
    };

    let mut json_files: Vec<PathBuf> = entries
        .filter_map(|res| res.ok())
        .filter(|entry| {
            let is_file = entry.file_type().map(|ft| ft.is_file()).unwrap_or(false);
            let is_json = entry.path().extension().is_some_and(|ext| ext == "json");
            is_file && is_json
        })
        .map(|entry| entry.path())
        .collect();

    if json_files.is_empty() {
        debug!("Skipping loading concepts: There are no additional concepts.");
        return Ok(Vec::new());
    }

    json_files.sort_unstable();
    json_files.dedup();

    Ok(json_files)
}

pub fn parse_concepts(file_paths: Vec<PathBuf>) -> color_eyre::Result<Vec<Concept>> {
    if file_paths.is_empty() {
        debug!("Skipping loading concepts: There are no additional concepts.");
        return Ok(Vec::new());
    }

    let concepts: Vec<Concept> = file_paths
        .par_iter()
        .filter_map(|path| {
            let name_or_path = path.file_name().map_or_else(|| path.as_path(), Path::new);

            let bytes = match fs::read(path) {
                Ok(bytes) => bytes,
                Err(error) => {
                    debug!("Failed to open {}: {error}", name_or_path.display());
                    return None;
                }
            };

            match serde_json::from_slice::<Concept>(&bytes) {
                Ok(concept) => Some(concept),
                Err(error) => {
                    debug!("Failed to parse {}: {error}", name_or_path.display());
                    None
                }
            }
        })
        .collect();

    Ok(concepts)
}
