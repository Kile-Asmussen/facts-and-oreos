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

/// The mods directory: `.factorio/mods/` relative to the project root.
pub fn mods_dir(project_root: &Path) -> PathBuf {
    project_root.join(".factorio").join("mods")
}

/// Resolve a set of requested mods (name + optional version constraint) to a
/// concrete list of (mod, version) pairs including all transitive required deps.
///
/// Uses a greedy latest-compatible strategy: for each mod, picks the newest
/// release satisfying all constraints seen so far.
pub async fn resolve(
    client: &ModPortalClient,
    requests: &[ModRequest],
) -> Result<Vec<ResolvedMod>> {
    let mut resolved: HashMap<String, ResolvedMod> = HashMap::new();
    // Stack of (mod_name, optional semver::Comparator constraint).
    let mut queue: Vec<(String, Option<semver::Comparator>)> = requests
        .iter()
        .map(|r| (r.name.clone(), r.version_constraint.clone()))
        .collect();
    // Track which mods declared incompatibilities.
    let mut incompatible: Vec<(String, String)> = Vec::new();

    while let Some((name, constraint)) = queue.pop() {
        if resolved.contains_key(&name) {
            // Already resolved — verify constraint is still satisfied.
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

        // Queue transitive required dependencies.
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

    // Check incompatibilities against resolved set.
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

/// Download resolved mods into the mods directory, skipping ones already cached.
/// Returns paths to all mod zips (downloaded or pre-existing).
pub async fn download(
    client: &ModPortalClient,
    token: &PlayerData,
    mods: &[ResolvedMod],
    mods_dir: &Path,
) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(mods_dir).context(Error::Io)?;
    let mut paths = Vec::new();

    for m in mods {
        let dest = mods_dir.join(&m.file_name);
        if dest.exists() && verify_sha1(&dest, &m.sha1) {
            paths.push(dest);
            continue;
        }

        let path = client
            .download_mod(&m.name, &m.version, token, mods_dir)
            .await
            .context(Error::Api)?;

        if !verify_sha1(&path, &m.sha1) {
            std::fs::remove_file(&path).ok();
            return Err(
                rootcause::report!(Error::Io).attach(format!("SHA1 mismatch for {}", m.file_name))
            );
        }

        paths.push(path);
    }

    Ok(paths)
}

fn verify_sha1(path: &Path, expected: &str) -> bool {
    let Ok(data) = std::fs::read(path) else {
        return false;
    };
    let digest = sha1_of(&data);
    digest == expected
}

fn sha1_of(data: &[u8]) -> String {
    // Simple SHA1 via the standard approach — we use the sha1 crate.
    use std::fmt::Write;
    let hash = sha1_smol::Sha1::from(data).digest().bytes();
    hash.iter().fold(String::with_capacity(40), |mut s, b| {
        write!(s, "{b:02x}").unwrap();
        s
    })
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
    let wrapper = ModListWrapper {
        mods: entries.to_vec(),
    };
    let data = serde_json::to_string_pretty(&wrapper).context(Error::Json)?;
    std::fs::write(mods_dir.join("modlist.json"), data).context(Error::Io)?;
    Ok(())
}

/// Merge resolved mods into an existing modlist, enabling them.
/// Preserves all existing entries; adds new ones; does not disable anything.
pub fn merge_into_modlist(existing: &mut Vec<ModListEntry>, resolved: &[ResolvedMod]) {
    for m in resolved {
        if !existing.iter().any(|e| e.name == m.name) {
            existing.push(ModListEntry {
                name: m.name.clone(),
                enabled: true,
            });
        } else {
            // Ensure it's enabled.
            for e in existing.iter_mut() {
                if e.name == m.name {
                    e.enabled = true;
                }
            }
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

/// A requested mod with an optional version constraint.
#[derive(Debug, Clone)]
pub struct ModRequest {
    pub name: String,
    pub version_constraint: Option<semver::Comparator>,
}

impl ModRequest {
    pub fn latest(name: impl Into<String>) -> Self {
        ModRequest {
            name: name.into(),
            version_constraint: None,
        }
    }

    pub fn with_constraint(name: impl Into<String>, constraint: semver::Comparator) -> Self {
        ModRequest {
            name: name.into(),
            version_constraint: Some(constraint),
        }
    }
}
