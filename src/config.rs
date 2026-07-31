use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub type Result<T> = rootcause::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Missing,
    Parse,
    Io,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Missing => write!(
                f,
                "no .facts-and-oreos.toml found — run `facts-and-oreos init` to initialise the project"
            ),
            Error::Parse => write!(f, "failed to parse .facts-and-oreos.toml"),
            Error::Io => write!(f, "I/O error reading config"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
struct RawConfig {
    factorio_bin: Option<PathBuf>,
    active_profile: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub factorio_bin: Option<PathBuf>,
    pub active_profile: String,
}

impl ProjectConfig {
    pub fn load(project_root: &Path) -> Result<Self> {
        let base_path = project_root.join(".facts-and-oreos.toml");
        if !base_path.exists() {
            return Err(rootcause::report!(Error::Missing));
        }

        let base: RawConfig = parse_file(&base_path)?;

        let local_path = project_root.join(".facts-and-oreos.local.toml");
        let merged = if local_path.exists() {
            let local: RawConfig = parse_file(&local_path)?;
            RawConfig {
                factorio_bin: local.factorio_bin.or(base.factorio_bin),
                active_profile: local.active_profile.or(base.active_profile),
            }
        } else {
            base
        };

        Ok(ProjectConfig {
            factorio_bin: merged.factorio_bin,
            active_profile: merged.active_profile.unwrap_or_else(|| "default".to_owned()),
        })
    }

    pub fn write(project_root: &Path, cfg: &Self) -> Result<()> {
        let raw = RawConfig {
            factorio_bin: cfg.factorio_bin.clone(),
            active_profile: Some(cfg.active_profile.clone()),
        };
        let toml_str = toml::to_string_pretty(&raw)
            .map_err(|_| rootcause::report!(Error::Parse))?;
        std::fs::write(project_root.join(".facts-and-oreos.toml"), toml_str)
            .map_err(|_| rootcause::report!(Error::Io))
    }
}

fn parse_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let text = std::fs::read_to_string(path).map_err(|_| rootcause::report!(Error::Io))?;
    toml::from_str(&text).map_err(|_| rootcause::report!(Error::Parse))
}
