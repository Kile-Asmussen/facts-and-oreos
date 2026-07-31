use std::path::{Path, PathBuf};

use rootcause::prelude::ResultExt as _;

use crate::{
    config::ProjectConfig,
    helper_mod,
    mod_settings::ModSettings,
};

use mod_api::credentials::PlayerData;

pub type Result<T> = rootcause::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    AlreadyExists(String),
    Io,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(name) => write!(
                f,
                "profile '{name}' does not exist — run `facts-and-oreos profile new {name}`"
            ),
            Error::AlreadyExists(name) => write!(f, "profile '{name}' already exists"),
            Error::Io => write!(f, "I/O error"),
        }
    }
}

pub struct Profile {
    pub name: String,
    pub root: PathBuf,
}

impl Profile {
    /// Resolve the active profile. Precedence:
    /// 1. `explicit` argument (from `--profile` flag)
    /// 2. `FAO_PROFILE` environment variable
    /// 3. `cfg.active_profile`
    ///
    /// Errors if the resolved profile directory does not exist.
    pub fn resolve(
        project_root: &Path,
        cfg: &ProjectConfig,
        explicit: Option<&str>,
    ) -> Result<Self> {
        let name = explicit
            .map(str::to_owned)
            .or_else(|| std::env::var("FAO_PROFILE").ok())
            .unwrap_or_else(|| cfg.active_profile.clone());

        let root = profiles_dir(project_root).join(&name);
        if !root.exists() {
            return Err(rootcause::report!(Error::NotFound(name)));
        }

        Ok(Profile { name, root })
    }

    pub fn mods_dir(&self) -> PathBuf {
        self.root.join("mods")
    }

    pub fn saves_dir(&self) -> PathBuf {
        self.root.join("saves")
    }

    pub fn mod_settings_path(&self) -> PathBuf {
        self.mods_dir().join("mod-settings.dat")
    }

    pub fn modlist_path(&self) -> PathBuf {
        self.mods_dir().join("modlist.json")
    }

    /// Path to the per-profile credentials file (gitignored, read-denied to Claude).
    pub fn player_data_path(&self) -> PathBuf {
        self.root.join("player-data.json")
    }

    /// Path to the committed mod manifest tracking physically-present zips.
    pub fn mod_records_path(&self) -> PathBuf {
        self.root.join("mods.json")
    }

    /// Create `mods/` and `saves/` directories.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.mods_dir())?;
        std::fs::create_dir_all(self.saves_dir())?;
        Ok(())
    }

    /// Write all embedded helper-mod files into `mods/facts-and-oreos-helper-mod/`.
    /// Idempotent: overwrites existing files.
    pub fn install_helper_mod(&self) -> Result<()> {
        let mod_root = self.mods_dir().join(helper_mod::MOD_DIR_NAME);
        for file in helper_mod::all_files() {
            let dest = mod_root.join(file.rel_path);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).context(Error::Io)?;
            }
            std::fs::write(&dest, file.contents).context(Error::Io)?;
        }
        Ok(())
    }

    /// Write a minimal `mod-settings.dat` into the profile if one does not exist yet.
    pub fn init_mod_settings(&self) -> Result<()> {
        let path = self.mod_settings_path();
        if path.exists() {
            return Ok(());
        }
        let settings = ModSettings::empty((2, 1, 12, 0));
        let bytes = settings
            .to_bytes()
            .map_err(|_| rootcause::report!(Error::Io))?;
        std::fs::write(&path, &bytes).context(Error::Io)
    }

    /// Write a minimal `modlist.json` enabling base and the helper mod.
    pub fn init_modlist(&self) -> Result<()> {
        let path = self.modlist_path();
        if path.exists() {
            return Ok(());
        }
        let modlist = serde_json::json!({
            "mods": [
                { "name": "base", "enabled": true },
                { "name": helper_mod::MOD_DIR_NAME, "enabled": true }
            ]
        });
        let text = serde_json::to_string_pretty(&modlist)
            .map_err(|_| rootcause::report!(Error::Io))?;
        std::fs::write(&path, text).context(Error::Io)
    }
}

/// Returns the `.profiles/` directory for the given project root.
pub fn profiles_dir(project_root: &Path) -> PathBuf {
    project_root.join(".profiles")
}

/// Load credentials from the profile's `player-data.json`.
pub fn load_player_data(profile: &Profile) -> Result<PlayerData> {
    let path = profile.player_data_path();
    if !path.exists() {
        return Err(rootcause::report!(Error::NotFound(format!(
            "no credentials for profile '{}' — run `facts-and-oreos set-token <username> <token>` (get your token at https://factorio.com/profile)",
            profile.name
        ))));
    }
    let text = std::fs::read_to_string(&path).context(Error::Io)?;
    serde_json::from_str(&text).map_err(|_| rootcause::report!(Error::Io))
}

/// Save credentials to the profile's `player-data.json` at mode 0o600.
pub fn save_player_data(profile: &Profile, data: &PlayerData) -> Result<()> {
    let path = profile.player_data_path();
    let text = serde_json::to_string_pretty(data).map_err(|_| rootcause::report!(Error::Io))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create(true).truncate(true).mode(0o600);
        let mut file = opts.open(&path).context(Error::Io)?;
        std::io::Write::write_all(&mut file, text.as_bytes()).context(Error::Io)?;
    }
    #[cfg(not(unix))]
    {
        std::fs::write(&path, text).context(Error::Io)?;
    }
    Ok(())
}

/// Create a new profile. Errors if it already exists.
pub fn create(project_root: &Path, name: &str) -> Result<Profile> {
    let root = profiles_dir(project_root).join(name);
    if root.exists() {
        return Err(rootcause::report!(Error::AlreadyExists(name.to_owned())));
    }
    let profile = Profile { name: name.to_owned(), root };
    profile.ensure_dirs().context(Error::Io)?;
    profile.install_helper_mod()?;
    profile.init_mod_settings()?;
    profile.init_modlist()?;
    Ok(profile)
}

/// List all profile names under `.profiles/`.
pub fn list(project_root: &Path) -> std::io::Result<Vec<String>> {
    let dir = profiles_dir(project_root);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut names: Vec<String> = std::fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    names.sort();
    Ok(names)
}
