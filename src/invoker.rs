use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

use rootcause::prelude::ResultExt as _;
#[allow(unused_imports)]
use crate::mod_settings::ModSettings;

pub type Result<T> = rootcause::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BinaryNotFound,
    Io,
    NonZeroExit { code: Option<i32>, stderr: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BinaryNotFound => write!(f, "Factorio binary not found; set FACTORIO_BIN or configure .factorio/project.toml"),
            Error::Io => write!(f, "I/O error"),
            Error::NonZeroExit { code, stderr } => {
                write!(f, "Factorio exited with code {code:?}")?;
                if !stderr.is_empty() {
                    write!(f, "\n{stderr}")?;
                }
                Ok(())
            }
        }
    }
}

/// Wraps a Factorio binary for headless invocations.
pub struct FactorioInvoker {
    binary: PathBuf,
    /// Path to Factorio's own data directory (contains core/ and base/).
    /// Derived from binary location if not set explicitly.
    read_data: PathBuf,
}

impl FactorioInvoker {
    /// Locate the Factorio binary. Checks `FACTORIO_BIN` env var first,
    /// then `.factorio/project.toml` in the project root (not yet implemented).
    pub fn from_env(project_root: &Path) -> Result<Self> {
        let binary = std::env::var("FACTORIO_BIN")
            .map(PathBuf::from)
            .or_else(|_| find_binary_from_project(project_root))
            .map_err(|_| rootcause::report!(Error::BinaryNotFound))?;

        if !binary.exists() {
            return Err(rootcause::report!(Error::BinaryNotFound));
        }

        // Factorio's data dir is typically <binary>/../data on Linux/Windows,
        // <binary>/../../data on macOS (inside .app bundle).
        let read_data = binary
            .parent()
            .and_then(|p| {
                let candidate = p.join("data");
                if candidate.exists() { return Some(candidate); }
                // macOS .app
                p.parent().map(|p2| p2.join("data"))
            })
            .filter(|p| p.exists())
            .ok_or_else(|| rootcause::report!(Error::BinaryNotFound)
                .attach("could not find Factorio data/ directory adjacent to binary"))?;

        Ok(FactorioInvoker { binary, read_data })
    }

    /// Run Factorio with `--dump-data` against the given mods directory.
    /// Returns the path to the written `data-raw-dump.json`.
    pub fn dump_data(&self, mods_dir: &Path) -> Result<PathBuf> {
        let tmp = self.make_isolated_env(mods_dir)?;
        let output = self.run(&tmp, &["--dump-data"])?;
        check_exit(&output)?;

        // Factorio writes script-output/data-raw-dump.json under write-data.
        let dump = tmp.write_data.join("script-output").join("data-raw-dump.json");
        if !dump.exists() {
            return Err(rootcause::report!(Error::Io)
                .attach("data-raw-dump.json not found after --dump-data"));
        }

        // Move the dump out of the temp dir to a stable location.
        let dest = mods_dir.join("data-raw-dump.json");
        std::fs::rename(&dump, &dest).context(Error::Io)?;
        Ok(dest)
    }

    /// Run a headless Factorio session.
    ///
    /// If `save` is `None`, loads the bundled `facts-and-oreos/empty` scenario.
    /// If `save` is `Some(path)`, loads that save file with `--load-game`.
    pub fn run_headless(
        &self,
        mods_dir: &Path,
        save: Option<&Path>,
        until_tick: u64,
    ) -> Result<RunOutput> {
        let tmp = self.make_isolated_env(mods_dir)?;

        let until = until_tick.to_string();
        let args: Vec<&str> = if let Some(save_path) = save {
            let save_str = save_path.to_str().unwrap_or("");
            vec!["--load-game", save_str, "--until-tick", &until]
        } else {
            vec!["--load-scenario", "facts-and-oreos/empty", "--until-tick", &until]
        };

        let output = self.run(&tmp, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        // Factorio exits non-zero in headless --until-tick runs even on success;
        // check the log for actual errors instead.
        let errors = parse_log_errors(&stdout);
        let warnings = parse_log_warnings(&stdout);

        Ok(RunOutput { stdout, stderr, errors, warnings })
    }

    /// Run `--dump-data` with the `facts-and-oreos` mod active, then parse
    /// the `defines` table out of script-output written by the on_init hook.
    pub fn dump_defines(&self, mods_dir: &Path) -> Result<PathBuf> {
        let tmp = self.make_isolated_env(mods_dir)?;
        // Use the minimal scenario which fires script.on_init to write defines.
        let output = self.run(&tmp, &[
            "--load-scenario", "facts-and-oreos/dump-defines",
            "--until-tick", "1",
        ])?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        let dest_src = tmp.write_data
            .join("script-output")
            .join("defines.json");

        if !dest_src.exists() {
            let errors = parse_log_errors(&stdout);
            return Err(rootcause::report!(Error::NonZeroExit {
                code: output.status.code(),
                stderr: errors.join("\n"),
            }));
        }

        let dest = mods_dir.join("defines.json");
        std::fs::rename(&dest_src, &dest).context(Error::Io)?;
        Ok(dest)
    }

    fn make_isolated_env(&self, mods_dir: &Path) -> Result<IsolatedEnv> {
        let tmp_dir = std::env::temp_dir().join(format!("facts-and-oreos-{}", std::process::id()));
        std::fs::create_dir_all(&tmp_dir).context(Error::Io)?;

        let write_data = tmp_dir.clone();
        std::fs::create_dir_all(write_data.join("mods")).context(Error::Io)?;

        // Write minimal mod-settings.dat.
        let settings = ModSettings::empty((2, 1, 12, 0));
        let settings_path = write_data.join("mods").join("mod-settings.dat");
        let bytes = settings.to_bytes().map_err(|_| rootcause::report!(Error::Io))?;
        std::fs::write(&settings_path, &bytes).context(Error::Io)?;

        // Write config.ini pointing at our read-data and write-data.
        let config_path = tmp_dir.join("config.ini");
        let config = format!(
            "[path]\nread-data={}\nwrite-data={}\n",
            self.read_data.display(),
            write_data.display(),
        );
        std::fs::write(&config_path, config).context(Error::Io)?;

        Ok(IsolatedEnv { write_data, config_path, mods_dir: mods_dir.to_path_buf() })
    }

    fn run(&self, env: &IsolatedEnv, extra_args: &[&str]) -> Result<Output> {
        let mut cmd = Command::new(&self.binary);
        cmd.arg("--config").arg(&env.config_path);
        cmd.arg("--mod-directory").arg(&env.mods_dir);
        cmd.args(extra_args);

        cmd.output().context(Error::Io)
    }
}

struct IsolatedEnv {
    write_data: PathBuf,
    config_path: PathBuf,
    mods_dir: PathBuf,
}

/// Structured output from a headless run.
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

fn check_exit(output: &Output) -> Result<()> {
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(rootcause::report!(Error::NonZeroExit {
            code: output.status.code(),
            stderr,
        }));
    }
    Ok(())
}

fn parse_log_errors(log: &str) -> Vec<String> {
    log.lines()
        .filter(|l| {
            let upper = l.to_uppercase();
            upper.contains(" ERROR ") || upper.starts_with("error")
        })
        .map(str::to_owned)
        .collect()
}

fn parse_log_warnings(log: &str) -> Vec<String> {
    log.lines()
        .filter(|l| {
            let upper = l.to_uppercase();
            upper.contains(" WARN ") || upper.contains(" WARNING ")
        })
        .map(str::to_owned)
        .collect()
}

fn find_binary_from_project(_project_root: &Path) -> std::result::Result<PathBuf, ()> {
    // TODO C1: read .factorio/project.toml for factorio-bin key
    Err(())
}
