use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize, de};
use semver::Version;

use crate::Error;

/// Full contents of a mod's `info.json` file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub contact: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub description: String,
    pub factorio_version: String,
    #[serde(default)]
    pub dependencies: Vec<ModDependency>,
}

/// One entry in `modlist.json`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModListEntry {
    pub name: String,
    pub enabled: bool,
}

/// A release of a mod as returned by the mod portal API.
#[derive(Clone, Debug, Deserialize)]
pub struct ModRelease {
    pub download_url: String,
    pub file_name: String,
    pub info_json: PortalManifest,
    pub released_at: String,
    #[serde(deserialize_with = "deserialize_version")]
    pub version: Version,
    pub sha1: String,
}

/// Minimal manifest embedded in portal API responses (not the full info.json).
#[derive(Clone, Debug, Deserialize)]
pub struct PortalManifest {
    pub factorio_version: String,
    #[serde(default)]
    pub dependencies: Vec<ModDependency>,
}

/// A mod entry as returned by the `/api/mods/{name}` portal endpoint.
#[derive(Debug, Deserialize)]
pub struct ModSpec {
    pub name: String,
    pub owner: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub description: Option<String>,
    pub releases: Vec<ModRelease>,
}

fn deserialize_version<'de, D: de::Deserializer<'de>>(d: D) -> std::result::Result<Version, D::Error> {
    let s = String::deserialize(d)?;
    // Factorio versions may have leading zeros in minor/patch components.
    let normalized = s
        .split('.')
        .map(|part| part.trim_start_matches('0').to_string())
        .map(|part| if part.is_empty() { "0".into() } else { part })
        .collect::<Vec<_>>()
        .join(".");
    Version::parse(&normalized).map_err(de::Error::custom)
}

/// The dependency prefix flavor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModDependencyFlavor {
    /// Required dependency.
    Normal,
    /// Incompatibility — must NOT be present.
    Incompatible,
    /// Optional — load after if present.
    Optional,
    /// Hidden optional.
    HiddenOptional,
    /// Load after if present, but does not affect load order.
    NoLoadOrder,
}

impl ModDependencyFlavor {
    pub fn is_required(&self) -> bool {
        matches!(self, ModDependencyFlavor::Normal)
    }
}

impl fmt::Display for ModDependencyFlavor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModDependencyFlavor::Normal => Ok(()),
            ModDependencyFlavor::Incompatible => write!(f, "!"),
            ModDependencyFlavor::Optional => write!(f, "?"),
            ModDependencyFlavor::HiddenOptional => write!(f, "(?)"),
            ModDependencyFlavor::NoLoadOrder => write!(f, "~"),
        }
    }
}

/// A single dependency specification, e.g. `"? some-mod >= 1.2.0"`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModDependency {
    pub flavor: ModDependencyFlavor,
    pub name: String,
    pub version_req: Option<semver::Comparator>,
}

impl ModDependency {
    pub fn required(name: String) -> Self {
        ModDependency { flavor: ModDependencyFlavor::Normal, name, version_req: None }
    }
}

impl fmt::Display for ModDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = self.flavor.to_string();
        if prefix.is_empty() {
            write!(f, "{}", self.name)?;
        } else {
            write!(f, "{} {}", prefix, self.name)?;
        }
        if let Some(req) = &self.version_req {
            let op = match req.op {
                semver::Op::Exact => "=",
                semver::Op::Greater => ">",
                semver::Op::GreaterEq => ">=",
                semver::Op::Less => "<",
                semver::Op::LessEq => "<=",
                _ => ">=",
            };
            write!(f, " {op} {}.{}.{}", req.major, req.minor.unwrap_or(0), req.patch.unwrap_or(0))?;
        }
        Ok(())
    }
}

impl FromStr for ModDependency {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Error> {
        let s = s.trim();

        let (flavor, rest) = if let Some(r) = s.strip_prefix("(?)") {
            (ModDependencyFlavor::HiddenOptional, r.trim_start())
        } else if let Some(r) = s.strip_prefix('!') {
            (ModDependencyFlavor::Incompatible, r.trim_start())
        } else if let Some(r) = s.strip_prefix('?') {
            (ModDependencyFlavor::Optional, r.trim_start())
        } else if let Some(r) = s.strip_prefix('~') {
            (ModDependencyFlavor::NoLoadOrder, r.trim_start())
        } else {
            (ModDependencyFlavor::Normal, s)
        };

        // Split name from optional version comparator.
        // Name chars: alphanumeric, hyphen, underscore, space (trimmed).
        // Comparator starts with one of: < <= = >= >
        let comparator_start = rest.find(|c| c == '<' || c == '=' || c == '>');

        let (name, version_req) = match comparator_start {
            None => (rest.trim().to_string(), None),
            Some(idx) => {
                let name = rest[..idx].trim().to_string();
                let comp_str = rest[idx..].trim();
                let req = semver::Comparator::parse(comp_str)
                    .map_err(|_| Error::InvalidDependency(s.to_string()))?;
                (name, Some(req))
            }
        };

        if name.is_empty() {
            return Err(Error::InvalidDependency(s.to_string()));
        }

        Ok(ModDependency { flavor, name, version_req })
    }
}

impl<'de> Deserialize<'de> for ModDependency {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl Serialize for ModDependency {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dep_normal() {
        let d: ModDependency = "base >= 1.1.0".parse().unwrap();
        assert_eq!(d.flavor, ModDependencyFlavor::Normal);
        assert_eq!(d.name, "base");
        assert!(d.version_req.is_some());
    }

    #[test]
    fn dep_optional() {
        let d: ModDependency = "? some-mod >= 4.2.0".parse().unwrap();
        assert_eq!(d.flavor, ModDependencyFlavor::Optional);
        assert_eq!(d.name, "some-mod");
    }

    #[test]
    fn dep_incompatible() {
        let d: ModDependency = "! bad-mod".parse().unwrap();
        assert_eq!(d.flavor, ModDependencyFlavor::Incompatible);
        assert!(d.version_req.is_none());
    }

    #[test]
    fn dep_hidden_optional() {
        let d: ModDependency = "(?) hidden-mod".parse().unwrap();
        assert_eq!(d.flavor, ModDependencyFlavor::HiddenOptional);
    }

    #[test]
    fn dep_no_load_order() {
        let d: ModDependency = "~ ordering-mod".parse().unwrap();
        assert_eq!(d.flavor, ModDependencyFlavor::NoLoadOrder);
    }

    #[test]
    fn dep_roundtrip() {
        let s = "? some-mod >= 4.2.0";
        let d: ModDependency = s.parse().unwrap();
        assert_eq!(d.to_string(), s);
    }
}
