pub mod credentials;
pub mod portal;
pub mod types;

pub use types::{ModDependency, ModDependencyFlavor, ModInfo, ModListEntry, ModRelease};

use std::fmt;

pub type Result<T> = rootcause::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidDependency(String),
    InvalidModName(String),
    InvalidVersion(String),
    ModVersionNotFound { name: String, version: semver::Version },
    LoginFailed { error: String, message: String },
    Http,
    Io,
    Json,
    CredentialStore,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidDependency(s) => write!(f, "invalid dependency string: {s:?}"),
            Error::InvalidModName(s) => write!(f, "invalid mod name: {s:?}"),
            Error::InvalidVersion(s) => write!(f, "invalid version string: {s:?}"),
            Error::ModVersionNotFound { name, version } => {
                write!(f, "mod {name:?} version {version} not found on portal")
            }
            Error::LoginFailed { error, message } => {
                write!(f, "login failed ({error}): {message}")
            }
            Error::Http => write!(f, "HTTP request failed"),
            Error::Io => write!(f, "I/O error"),
            Error::Json => write!(f, "JSON parse error"),
            Error::CredentialStore => write!(f, "credential store error"),
        }
    }
}
