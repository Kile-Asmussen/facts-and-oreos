use std::path::PathBuf;

use rootcause::prelude::ResultExt as _;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// A Factorio mod portal API token. Never implement Debug — prevents accidental logging.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlayerData {
    pub service_username: String,
    pub service_token: String,
}

fn player_data_path() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| ".".into())
        .join(".factorio")
        .join("player-data.json")
}

pub fn load() -> Result<Option<PlayerData>> {
    let path = player_data_path();
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&path).context(Error::CredentialStore)?;
    let token = serde_json::from_str(&data).context(Error::CredentialStore)?;
    Ok(Some(token))
}

pub fn save(token: &PlayerData) -> Result<()> {
    let path = player_data_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context(Error::CredentialStore)?;
    }
    let data = serde_json::to_string_pretty(token).context(Error::CredentialStore)?;
    // Permissions: owner-read-only (600) on Unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create(true).truncate(true).mode(0o600);
        let mut file = opts.open(&path).context(Error::CredentialStore)?;
        std::io::Write::write_all(&mut file, data.as_bytes()).context(Error::CredentialStore)?;
    }
    #[cfg(not(unix))]
    {
        std::fs::write(&path, data).context(Error::CredentialStore)?;
    }
    Ok(())
}

pub fn clear() -> Result<()> {
    let path = player_data_path();
    if path.exists() {
        std::fs::remove_file(&path).context(Error::CredentialStore)?;
    }
    Ok(())
}
