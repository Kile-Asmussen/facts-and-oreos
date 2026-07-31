use std::path::{Path, PathBuf};

use rootcause::prelude::ResultExt as _;
use semver::Version;
use serde::Deserialize;

use crate::{Error, Result, credentials::PlayerData, types::ModSpec};

const MOD_API_BASE: &str = "https://mods.factorio.com";
const AUTH_API_BASE: &str = "https://auth.factorio.com";

pub struct ModPortalClient {
    client: reqwest::Client,
    mod_api_base: String,
    auth_api_base: String,
}

impl ModPortalClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("facts-and-oreos/0.1")
            .build()
            .context(Error::Http)?;
        Ok(Self {
            client,
            mod_api_base: MOD_API_BASE.into(),
            auth_api_base: AUTH_API_BASE.into(),
        })
    }

    /// Fetch the full spec for a mod from the portal.
    pub async fn get_mod_spec(&self, name: &str) -> Result<ModSpec> {
        validate_mod_name(name)?;
        let url = format!("{}/api/mods/{name}", self.mod_api_base);
        let spec = self
            .client
            .get(&url)
            .send()
            .await
            .context(Error::Http)?
            .error_for_status()
            .context(Error::Http)?
            .json::<ModSpec>()
            .await
            .context(Error::Json)?;
        Ok(spec)
    }

    /// Log in and return an API token. Store it with `credentials::save` if desired.
    pub async fn login(&self, username: &str, password: &str) -> Result<PlayerData> {
        let url = format!("{}/api-login", self.auth_api_base);
        let params = [
            ("api_version", "4"),
            ("username", username),
            ("password", password),
        ];
        let response: LoginResponse = self
            .client
            .post(&url)
            .query(&params)
            .send()
            .await
            .context(Error::Http)?
            .json()
            .await
            .context(Error::Json)?;

        match response {
            LoginResponse::Success {
                token: service_token,
                username: service_username,
            } => Ok(PlayerData {
                service_username,
                service_token,
            }),
            LoginResponse::Error { error, message } => {
                Err(rootcause::report!(Error::LoginFailed { error, message }))
            }
        }
    }

    /// Download a mod directly by URL (no extra `get_mod_spec` call).
    /// `url` is the path component from the portal API (e.g. `/download/...`).
    /// Writes the file to `dest` (full path including filename).
    pub async fn download_mod_by_url(
        &self,
        url: &str,
        _file_name: &str,
        token: &PlayerData,
        dest: &Path,
    ) -> Result<PathBuf> {
        let full_url = if url.starts_with("http") {
            url.to_owned()
        } else {
            format!("{}{url}", self.mod_api_base)
        };
        let params = [
            ("username", &token.service_username),
            ("token", &token.service_token),
        ];
        let response = self
            .client
            .get(&full_url)
            .query(&params)
            .send()
            .await
            .context(Error::Http)?
            .error_for_status()
            .context(Error::Http)?;

        let bytes = response.bytes().await.context(Error::Http)?;
        std::fs::write(dest, &bytes).context(Error::Io)?;
        Ok(dest.to_path_buf())
    }

    /// Download a specific mod version to `directory`. Returns the path to the zip file.
    pub async fn download_mod(
        &self,
        name: &str,
        version: &Version,
        token: &PlayerData,
        directory: &Path,
    ) -> Result<PathBuf> {
        validate_mod_name(name)?;
        let spec = self.get_mod_spec(name).await?;
        let release = spec
            .releases
            .iter()
            .find(|r| &r.version == version)
            .ok_or_else(|| {
                rootcause::report!(Error::ModVersionNotFound {
                    name: name.into(),
                    version: version.clone(),
                })
            })?;

        let url = format!("{}{}", self.mod_api_base, release.download_url);
        let params = [
            ("username", &token.service_username),
            ("token", &token.service_token),
        ];

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .context(Error::Http)?
            .error_for_status()
            .context(Error::Http)?;

        let dest = directory.join(&release.file_name);
        let bytes = response.bytes().await.context(Error::Http)?;
        std::fs::write(&dest, &bytes).context(Error::Io)?;

        Ok(dest)
    }
}

fn validate_mod_name(name: &str) -> Result<()> {
    if name.is_empty() || name.contains('/') || name.contains('\\') {
        return Err(rootcause::report!(Error::InvalidModName(name.into())));
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(untagged)]
enum LoginResponse {
    Success { token: String, username: String },
    Error { error: String, message: String },
}
