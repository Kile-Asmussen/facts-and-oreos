use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use mod_api::{
    ModDependencyFlavor, ModListEntry,
    credentials::PlayerData,
    portal::ModPortalClient,
    types::{ModInfo, ModRelease, ModSpec},
};
use rootcause::prelude::ResultExt as _;
use semver::Version;

pub type Result<T> = rootcause::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ModNotFound(String),
    NoCompatibleVersion { name: String, constraint: String },
    CyclicDependency(String),
    Incompatible { mod_a: String, mod_b: String },
    Io,
    Json,
    Zip,
    Api,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ModNotFound(n) => write!(f, "mod not found: {n}"),
            Error::NoCompatibleVersion { name, constraint } => {
                write!(f, "no version of {name} satisfies {constraint}")
            }
            Error::CyclicDependency(n) => write!(f, "cyclic dependency involving {n}"),
            Error::Incompatible { mod_a, mod_b } => {
                write!(f, "mods {mod_a} and {mod_b} are incompatible")
            }
            Error::Io => write!(f, "I/O error"),
            Error::Json => write!(f, "JSON parse/write error"),
            Error::Zip => write!(f, "zip extraction error"),
            Error::Api => write!(f, "mod portal API error"),
        }
    }
}

/// A resolved (name, version, download_url, file_name, sha1) tuple.
#[derive(Debug, Clone)]
pub struct ResolvedMod {
    pub name: String,
    pub version: Version,
    pub download_url: String,
    pub file_name: String,
    pub sha1: String,
}

/// A record of a physically-present mod zip, stored in `mods.json` alongside the profile.
/// Allows the `mods/` directory to be gitignored while keeping a committed manifest.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModRecord {
    pub name: String,
    pub version: Version,
    pub file_name: String,
    pub sha1: String,
}

/// Shared zip cache directory: `<project_root>/.cache/mods/`.
pub fn cache_dir(project_root: &Path) -> PathBuf {
    project_root.join(".cache").join("mods")
}

/// Resolve a set of requested mods (name + optional version constraint) to a
/// concrete list of (mod, version) pairs including all transitive required deps.
pub async fn resolve(
    client: &ModPortalClient,
    requests: &[ModRequest],
) -> Result<Vec<ResolvedMod>> {
    let mut resolved: HashMap<String, ResolvedMod> = HashMap::new();
    let mut queue: Vec<(String, Option<semver::Comparator>)> = requests
        .iter()
        .map(|r| (r.name.clone(), r.version_constraint.clone()))
        .collect();
    let mut incompatible: Vec<(String, String)> = Vec::new();

    while let Some((name, constraint)) = queue.pop() {
        if resolved.contains_key(&name) {
            let existing = &resolved[&name];
            if let Some(ref c) = constraint {
                if !c.matches(&existing.version) {
                    return Err(rootcause::report!(Error::NoCompatibleVersion {
                        name,
                        constraint: c.to_string(),
                    }));
                }
            }
            continue;
        }

        let spec = client.get_mod_spec(&name).await.context(Error::Api)?;

        let release = pick_release(&spec, constraint.as_ref()).ok_or_else(|| {
            rootcause::report!(Error::NoCompatibleVersion {
                name: name.clone(),
                constraint: constraint
                    .as_ref()
                    .map(|c| c.to_string())
                    .unwrap_or_default(),
            })
        })?;

        for dep in &release.info_json.dependencies {
            match dep.flavor {
                ModDependencyFlavor::Normal => {
                    queue.push((dep.name.clone(), dep.version_req.clone()));
                }
                ModDependencyFlavor::Incompatible => {
                    incompatible.push((name.clone(), dep.name.clone()));
                }
                _ => {}
            }
        }

        resolved.insert(
            name.clone(),
            ResolvedMod {
                name: name.clone(),
                version: release.version.clone(),
                download_url: release.download_url.clone(),
                file_name: release.file_name.clone(),
                sha1: release.sha1.clone(),
            },
        );
    }

    for (declarer, banned) in &incompatible {
        if resolved.contains_key(banned.as_str()) {
            return Err(rootcause::report!(Error::Incompatible {
                mod_a: declarer.clone(),
                mod_b: banned.clone(),
            }));
        }
    }

    Ok(resolved.into_values().collect())
}

fn pick_release<'a>(
    spec: &'a ModSpec,
    constraint: Option<&semver::Comparator>,
) -> Option<&'a ModRelease> {
    spec.releases
        .iter()
        .filter(|r| constraint.map_or(true, |c| c.matches(&r.version)))
        .max_by(|a, b| a.version.cmp(&b.version))
}

/// Download resolved mods into `mods_dir`, using `cache_dir` as a shared zip cache.
///
/// For each mod:
/// 1. If already in `mods_dir` with valid SHA1, skip.
/// 2. If in `cache_dir` with valid SHA1, copy from cache.
/// 3. Otherwise download from portal, write to cache, then copy to `mods_dir`.
///
/// Pass `cache_dir: None` to disable caching (useful for tests).
pub async fn download(
    client: &ModPortalClient,
    token: &PlayerData,
    mods: &[ResolvedMod],
    mods_dir: &Path,
    cache_dir: Option<&Path>,
) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(mods_dir).context(Error::Io)?;
    if let Some(c) = cache_dir {
        std::fs::create_dir_all(c).context(Error::Io)?;
    }

    let mut paths = Vec::new();

    for m in mods {
        let dest = mods_dir.join(&m.file_name);

        // Already present in mods dir with correct hash.
        if dest.exists() && verify_sha1(&dest, &m.sha1) {
            paths.push(dest);
            continue;
        }

        // Try the shared cache.
        if let Some(cache) = cache_dir {
            let cached = cache.join(&m.file_name);
            if cached.exists() {
                if verify_sha1(&cached, &m.sha1) {
                    std::fs::copy(&cached, &dest).context(Error::Io)?;
                    paths.push(dest);
                    continue;
                } else {
                    // Corrupt cache entry — remove and re-fetch.
                    std::fs::remove_file(&cached).ok();
                }
            }
        }

        // Download from portal into cache (or directly to mods_dir if no cache).
        let download_dest = match cache_dir {
            Some(cache) => cache.join(&m.file_name),
            None => dest.clone(),
        };

        client
            .download_mod_by_url(&m.download_url, &m.file_name, token, &download_dest)
            .await
            .context(Error::Api)?;

        if !verify_sha1(&download_dest, &m.sha1) {
            std::fs::remove_file(&download_dest).ok();
            return Err(rootcause::report!(Error::Io)
                .attach(format!("SHA1 mismatch for {}", m.file_name)));
        }

        // Copy from cache into mods_dir (no-op if cache_dir is None, dest == download_dest).
        if cache_dir.is_some() {
            std::fs::copy(&download_dest, &dest).context(Error::Io)?;
        }

        paths.push(dest);
    }

    Ok(paths)
}

fn verify_sha1(path: &Path, expected: &str) -> bool {
    let Ok(data) = std::fs::read(path) else {
        return false;
    };
    sha1_of(&data) == expected
}

fn sha1_of(data: &[u8]) -> String {
    use sha1::{Digest, Sha1};
    hex::encode(Sha1::digest(data))
}

/// Read `modlist.json` from the mods directory.
pub fn read_modlist(mods_dir: &Path) -> Result<Vec<ModListEntry>> {
    let path = mods_dir.join("modlist.json");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(&path).context(Error::Io)?;
    let wrapper: ModListWrapper = serde_json::from_str(&data).context(Error::Json)?;
    Ok(wrapper.mods)
}

/// Write `modlist.json` to the mods directory.
pub fn write_modlist(mods_dir: &Path, entries: &[ModListEntry]) -> Result<()> {
    std::fs::create_dir_all(mods_dir).context(Error::Io)?;
    let wrapper = ModListWrapper { mods: entries.to_vec() };
    let data = serde_json::to_string_pretty(&wrapper).context(Error::Json)?;
    std::fs::write(mods_dir.join("modlist.json"), data).context(Error::Io)?;
    Ok(())
}

/// Merge resolved mods into an existing modlist, enabling them.
pub fn merge_into_modlist(existing: &mut Vec<ModListEntry>, resolved: &[ResolvedMod]) {
    for m in resolved {
        if let Some(entry) = existing.iter_mut().find(|e| e.name == m.name) {
            entry.enabled = true;
        } else {
            existing.push(ModListEntry { name: m.name.clone(), enabled: true });
        }
    }
}

/// Read `mods.json` from the profile root (one level above `mods/`).
pub fn read_mod_records(profile_root: &Path) -> Result<Vec<ModRecord>> {
    let path = profile_root.join("mods.json");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(&path).context(Error::Io)?;
    let wrapper: ModRecordWrapper = serde_json::from_str(&data).context(Error::Json)?;
    Ok(wrapper.mods)
}

/// Write `mods.json` to the profile root.
pub fn write_mod_records(profile_root: &Path, records: &[ModRecord]) -> Result<()> {
    let wrapper = ModRecordWrapper { mods: records.to_vec() };
    let data = serde_json::to_string_pretty(&wrapper).context(Error::Json)?;
    std::fs::write(profile_root.join("mods.json"), data).context(Error::Io)?;
    Ok(())
}

/// Upsert resolved mods into the existing records list (matched by name).
pub fn merge_mod_records(existing: &mut Vec<ModRecord>, resolved: &[ResolvedMod]) {
    for m in resolved {
        if let Some(rec) = existing.iter_mut().find(|r| r.name == m.name) {
            rec.version = m.version.clone();
            rec.file_name = m.file_name.clone();
            rec.sha1 = m.sha1.clone();
        } else {
            existing.push(ModRecord {
                name: m.name.clone(),
                version: m.version.clone(),
                file_name: m.file_name.clone(),
                sha1: m.sha1.clone(),
            });
        }
    }
}

/// Read `info.json` from an unpacked mod directory.
pub fn read_mod_info(mod_dir: &Path) -> Result<ModInfo> {
    let data = std::fs::read_to_string(mod_dir.join("info.json")).context(Error::Io)?;
    serde_json::from_str(&data).context(Error::Json)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ModListWrapper {
    mods: Vec<ModListEntry>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ModRecordWrapper {
    mods: Vec<ModRecord>,
}

/// A requested mod with an optional version constraint.
#[derive(Debug, Clone)]
pub struct ModRequest {
    pub name: String,
    pub version_constraint: Option<semver::Comparator>,
}

impl ModRequest {
    pub fn latest(name: impl Into<String>) -> Self {
        ModRequest { name: name.into(), version_constraint: None }
    }

    pub fn with_constraint(name: impl Into<String>, constraint: semver::Comparator) -> Self {
        ModRequest { name: name.into(), version_constraint: Some(constraint) }
    }
}
