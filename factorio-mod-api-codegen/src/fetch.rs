use std::path::{Path, PathBuf};

const API_VERSION: &str = "2.1.12";
const BASE_URL: &str = "https://lua-api.factorio.com";

/// Returns the path to prototype-api.json under `cache_dir`, downloading it if absent.
pub fn prototype_api_json(cache_dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    fetch_api_file(cache_dir, "prototype-api.json")
}

/// Returns the path to runtime-api.json under `cache_dir`, downloading it if absent.
pub fn runtime_api_json(cache_dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    fetch_api_file(cache_dir, "runtime-api.json")
}

fn fetch_api_file(cache_dir: &Path, filename: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dest = cache_dir.join(filename);
    if dest.exists() {
        return Ok(dest);
    }

    std::fs::create_dir_all(cache_dir)?;

    let url = format!("{BASE_URL}/{API_VERSION}/{filename}");
    eprintln!("Downloading {url} ...");
    let bytes = reqwest::blocking::get(&url)?.error_for_status()?.bytes()?;
    std::fs::write(&dest, &bytes)?;
    eprintln!("Saved {} ({} bytes)", dest.display(), bytes.len());

    Ok(dest)
}
