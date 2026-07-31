use std::path::{Path, PathBuf};
use std::process::Command;

use clap::{Args, Parser, Subcommand};
use facts_and_oreos::{
    check_token,
    config::ProjectConfig,
    downloader::{self, ModRequest},
    invoker::FactorioInvoker,
    profile::{self, Profile},
};
use mod_api::{credentials::PlayerData, portal::ModPortalClient};
use rootcause::prelude::ResultExt as _;

type Result<T> = rootcause::Result<T, CliError>;

#[derive(Debug)]
enum CliError {
    BadArg(String),
    Config,
    Profile,
    Api,
    Downloader,
    Invoker,
    CheckToken,
    Io,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::BadArg(msg)   => write!(f, "{msg}"),
            CliError::Config        => write!(f, "config error"),
            CliError::Profile       => write!(f, "profile error"),
            CliError::Api           => write!(f, "mod portal API error"),
            CliError::Downloader    => write!(f, "downloader error"),
            CliError::Invoker       => write!(f, "Factorio invoker error"),
            CliError::CheckToken    => write!(f, "token scanner error"),
            CliError::Io            => write!(f, "I/O error"),
        }
    }
}

// ---------------------------------------------------------------------------
// CLI structure
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(name = "facts-and-oreos", about = "Factorio mod development tool suite")]
struct Cli {
    #[command(flatten)]
    common: CommonArgs,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Args)]
struct CommonArgs {
    #[arg(long, global = true, env = "FAO_PROFILE", help = "Override the active profile")]
    profile: Option<String>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Initialise project, create default profile
    Init(InitArgs),
    /// Scan for Factorio service tokens (30-char hex)
    CheckToken(CheckTokenArgs),
    /// Resolve and download mods into active profile
    Fetch(FetchArgs),
    /// Store mod portal credentials for active profile
    SetToken(SetTokenArgs),
    /// Run Factorio headless (scenario or save)
    Run(RunArgs),
    /// Dump data.raw as JSON
    DumpData,
    /// Dump defines table as JSON
    DumpDefines,
    /// Profile management
    Profile(ProfileCmd),
}

// ---------------------------------------------------------------------------
// Subcommand arg structs
// ---------------------------------------------------------------------------

#[derive(Args)]
struct InitArgs {
    /// Path to the Factorio binary
    #[arg(long)]
    factorio_bin: Option<PathBuf>,
    /// Initialise a git repo and install pre-commit hook
    #[arg(long)]
    git: bool,
    /// Reinitialise even if config already exists
    #[arg(long)]
    force: bool,
}

#[derive(Args)]
struct CheckTokenArgs {
    /// Scan staged git blobs instead of files/stdin
    #[arg(long)]
    staged: bool,
    /// Files to scan (reads stdin if empty and --staged not set)
    files: Vec<PathBuf>,
}

#[derive(Args)]
struct FetchArgs {
    /// Mod names to fetch (with all transitive dependencies)
    #[arg(required = true)]
    mods: Vec<String>,
}

#[derive(Args)]
struct SetTokenArgs {
    /// Factorio account username
    username: String,
    /// API token from https://factorio.com/profile
    token: String,
}

#[derive(Args)]
struct RunArgs {
    /// Save file to load (relative to profile saves dir, or absolute)
    save: Option<PathBuf>,
}

#[derive(Args)]
struct ProfileCmd {
    #[command(subcommand)]
    sub: ProfileSub,
}

#[derive(Subcommand)]
enum ProfileSub {
    /// List profiles, marking the active one
    List,
    /// Create a new profile
    New { name: String },
    /// Set the active profile in config
    SetDefault { name: String },
    /// Delete a profile
    Delete {
        name: String,
        /// Delete even if profile contains mod zips
        #[arg(long)]
        force: bool,
    },
    /// Clone a profile (mods only, not saves)
    Clone { src: String, dst: String },
    /// Move mods from old .factorio/mods/ layout into a profile
    Migrate,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let project_root = PathBuf::from(".");

    let result = match cli.command {
        Cmd::Init(a)        => cmd_init(&project_root, &a),
        Cmd::CheckToken(a)  => cmd_check_token(&project_root, &a),
        Cmd::Fetch(a)       => cmd_fetch(&project_root, cli.common.profile.as_deref(), &a).await,
        Cmd::SetToken(a)    => cmd_set_token(&project_root, cli.common.profile.as_deref(), &a),
        Cmd::Run(a)         => cmd_run(&project_root, cli.common.profile.as_deref(), &a),
        Cmd::DumpData       => cmd_dump_data(&project_root, cli.common.profile.as_deref()),
        Cmd::DumpDefines    => cmd_dump_defines(&project_root, cli.common.profile.as_deref()),
        Cmd::Profile(a)     => cmd_profile(&project_root, cli.common.profile.as_deref(), &a).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_config(project_root: &Path) -> Result<ProjectConfig> {
    ProjectConfig::load(project_root).context(CliError::Config)
}

fn resolve_profile(project_root: &Path, cfg: &ProjectConfig, explicit: Option<&str>) -> Result<Profile> {
    Profile::resolve(project_root, cfg, explicit).context(CliError::Profile)
}

// ---------------------------------------------------------------------------
// init
// ---------------------------------------------------------------------------

fn cmd_init(project_root: &Path, args: &InitArgs) -> Result<()> {
    let config_path = project_root.join(".facts-and-oreos.toml");
    if config_path.exists() && !args.force {
        return Err(rootcause::report!(CliError::BadArg(
            ".facts-and-oreos.toml already exists. Use --force to reinitialise.".into()
        )));
    }

    let cfg = ProjectConfig {
        factorio_bin: args.factorio_bin.clone(),
        active_profile: "default".to_owned(),
    };
    ProjectConfig::write(project_root, &cfg).context(CliError::Config)?;
    eprintln!("Wrote .facts-and-oreos.toml");

    if !project_root.join(".profiles").join("default").exists() {
        profile::create(project_root, "default").context(CliError::Profile)?;
        eprintln!("Created profile 'default' at .profiles/default/");
    }

    for entry in &[
        ".facts-and-oreos.local.toml",
        "**/player-data.json",
        ".profiles/*/saves/",
        ".profiles/*/mods/",
        ".cache/",
    ] {
        ensure_gitignore(project_root, entry).context(CliError::Io)?;
    }
    eprintln!("Patched .gitignore");

    if args.git {
        install_git_hook(project_root)?;
    }

    if args.factorio_bin.is_none() && std::env::var("FACTORIO_BIN").is_err() {
        eprintln!();
        eprintln!("Note: no factorio-bin configured. Set it in .facts-and-oreos.toml or via FACTORIO_BIN before using invoke commands.");
    }

    Ok(())
}

fn install_git_hook(project_root: &Path) -> Result<()> {
    let git_dir = project_root.join(".git");

    if !git_dir.exists() {
        let status = Command::new("git")
            .arg("init")
            .current_dir(project_root)
            .status()
            .context(CliError::Io)?;
        if !status.success() {
            return Err(rootcause::report!(CliError::BadArg("git init failed".into())));
        }
        eprintln!("Initialised git repository.");
    }

    let hooks_dir = {
        let out = Command::new("git")
            .args(["config", "core.hooksPath"])
            .current_dir(project_root)
            .output()
            .context(CliError::Io)?;
        if out.status.success() {
            let p = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            if !p.is_empty() {
                let hp = PathBuf::from(&p);
                if hp.is_absolute() { hp } else { project_root.join(hp) }
            } else {
                git_dir.join("hooks")
            }
        } else {
            git_dir.join("hooks")
        }
    };

    std::fs::create_dir_all(&hooks_dir).context(CliError::Io)?;
    let hook_path = hooks_dir.join("pre-commit");
    let hook_content = "#!/bin/sh\n\
        # Installed by facts-and-oreos init --git\n\
        # Requires facts-and-oreos to be on $PATH.\n\
        facts-and-oreos check-token --staged\n";
    std::fs::write(&hook_path, hook_content).context(CliError::Io)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&hook_path).context(CliError::Io)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&hook_path, perms).context(CliError::Io)?;
    }

    eprintln!("Installed pre-commit hook at {}", hook_path.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// check-token
// ---------------------------------------------------------------------------

fn cmd_check_token(project_root: &Path, args: &CheckTokenArgs) -> Result<()> {
    let matches = if args.staged {
        check_token::scan_staged(project_root).context(CliError::CheckToken)?
    } else if args.files.is_empty() {
        let stdin = std::io::stdin();
        check_token::scan_reader(stdin.lock(), "<stdin>")
    } else {
        let mut all = Vec::new();
        for path in &args.files {
            all.extend(check_token::scan_file(path).context(CliError::CheckToken)?);
        }
        all
    };

    if matches.is_empty() {
        return Ok(());
    }
    for m in &matches {
        eprintln!("{}:{}: token detected", m.file, m.line);
    }
    std::process::exit(1);
}

// ---------------------------------------------------------------------------
// profile subcommands
// ---------------------------------------------------------------------------

async fn cmd_profile(project_root: &Path, profile_override: Option<&str>, cmd: &ProfileCmd) -> Result<()> {
    match &cmd.sub {
        ProfileSub::List => cmd_profile_list(project_root, profile_override),
        ProfileSub::New { name } => cmd_profile_new(project_root, name),
        ProfileSub::SetDefault { name } => cmd_profile_set_default(project_root, name),
        ProfileSub::Delete { name, force } => cmd_profile_delete(project_root, name, *force),
        ProfileSub::Clone { src, dst } => cmd_profile_clone(project_root, src, dst),
        ProfileSub::Migrate => cmd_profile_migrate(project_root, profile_override),
    }
}

fn cmd_profile_list(project_root: &Path, profile_override: Option<&str>) -> Result<()> {
    let cfg = load_config(project_root)?;
    let active = profile_override
        .map(str::to_owned)
        .or_else(|| std::env::var("FAO_PROFILE").ok())
        .unwrap_or_else(|| cfg.active_profile.clone());

    let names = profile::list(project_root).context(CliError::Io)?;
    if names.is_empty() {
        eprintln!("No profiles found. Run `facts-and-oreos profile new <name>`.");
        return Ok(());
    }
    for name in &names {
        if name == &active { eprintln!("* {name}"); } else { eprintln!("  {name}"); }
    }
    Ok(())
}

fn cmd_profile_new(project_root: &Path, name: &str) -> Result<()> {
    let _ = load_config(project_root)?;
    profile::create(project_root, name).context(CliError::Profile)?;
    eprintln!("Created profile '{name}'.");
    Ok(())
}

fn cmd_profile_set_default(project_root: &Path, name: &str) -> Result<()> {
    let mut cfg = load_config(project_root)?;
    Profile::resolve(project_root, &cfg, Some(name)).context(CliError::Profile)?;
    cfg.active_profile = name.to_owned();
    ProjectConfig::write(project_root, &cfg).context(CliError::Config)?;
    eprintln!("Active profile set to '{name}'.");
    Ok(())
}

fn cmd_profile_delete(project_root: &Path, name: &str, force: bool) -> Result<()> {
    let cfg = load_config(project_root)?;
    if name == cfg.active_profile {
        return Err(rootcause::report!(CliError::BadArg(format!(
            "cannot delete the active profile '{name}'. Switch first with `profile set-default`."
        ))));
    }

    let root = profile::profiles_dir(project_root).join(name);
    if !root.exists() {
        return Err(rootcause::report!(CliError::BadArg(format!(
            "profile '{name}' does not exist"
        ))));
    }

    if !force {
        let has_zips = std::fs::read_dir(root.join("mods"))
            .map(|rd| rd.filter_map(|e| e.ok()).any(|e| e.file_name().to_string_lossy().ends_with(".zip")))
            .unwrap_or(false);
        if has_zips {
            return Err(rootcause::report!(CliError::BadArg(format!(
                "profile '{name}' contains mod zips. Use --force to delete anyway."
            ))));
        }
    }

    std::fs::remove_dir_all(&root).context(CliError::Io)?;
    eprintln!("Deleted profile '{name}'.");
    Ok(())
}

fn cmd_profile_clone(project_root: &Path, src_name: &str, dst_name: &str) -> Result<()> {
    let _ = load_config(project_root)?;
    let src_root = profile::profiles_dir(project_root).join(src_name);
    if !src_root.exists() {
        return Err(rootcause::report!(CliError::BadArg(format!(
            "source profile '{src_name}' does not exist"
        ))));
    }
    let dst_root = profile::profiles_dir(project_root).join(dst_name);
    if dst_root.exists() {
        return Err(rootcause::report!(CliError::BadArg(format!(
            "destination profile '{dst_name}' already exists"
        ))));
    }

    let dst = Profile { name: dst_name.to_owned(), root: dst_root };
    dst.ensure_dirs().context(CliError::Io)?;
    dst.install_helper_mod().context(CliError::Profile)?;
    dst.init_mod_settings().context(CliError::Profile)?;

    let src_modlist = src_root.join("mods").join("modlist.json");
    if src_modlist.exists() {
        std::fs::copy(&src_modlist, dst.modlist_path()).context(CliError::Io)?;
    } else {
        dst.init_modlist().context(CliError::Profile)?;
    }

    let src_records = src_root.join("mods.json");
    if src_records.exists() {
        std::fs::copy(&src_records, dst.mod_records_path()).context(CliError::Io)?;
    }

    let mut copied = 0usize;
    let src_mods = src_root.join("mods");
    if src_mods.exists() {
        for entry in std::fs::read_dir(&src_mods).context(CliError::Io)? {
            let entry = entry.context(CliError::Io)?;
            let fname = entry.file_name();
            let fname_str = fname.to_string_lossy();
            if fname_str.ends_with(".zip") {
                std::fs::copy(entry.path(), dst.mods_dir().join(&*fname_str)).context(CliError::Io)?;
                copied += 1;
            }
        }
    }

    eprintln!("Cloned profile '{src_name}' → '{dst_name}' ({copied} mod zip(s) copied).");
    Ok(())
}

fn cmd_profile_migrate(project_root: &Path, profile_override: Option<&str>) -> Result<()> {
    let cfg = load_config(project_root)?;
    let prof = resolve_profile(project_root, &cfg, profile_override)?;

    let old_mods = project_root.join(".factorio").join("mods");
    if !old_mods.exists() {
        eprintln!("No .factorio/mods/ directory found; nothing to migrate.");
        return Ok(());
    }

    let mut moved = 0usize;
    for entry in std::fs::read_dir(&old_mods).context(CliError::Io)? {
        let entry = entry.context(CliError::Io)?;
        let fname = entry.file_name();
        let fname_str = fname.to_string_lossy();
        if fname_str.ends_with(".zip") {
            std::fs::rename(entry.path(), prof.mods_dir().join(&*fname_str)).context(CliError::Io)?;
            eprintln!("  Moved {fname_str}");
            moved += 1;
        }
    }

    let old_modlist = old_mods.join("modlist.json");
    if old_modlist.exists() && !prof.modlist_path().exists() {
        std::fs::rename(&old_modlist, prof.modlist_path()).context(CliError::Io)?;
        eprintln!("  Moved modlist.json");
    }

    eprintln!("Migrated {moved} mod zip(s) into profile '{}'.", prof.name);
    Ok(())
}

// ---------------------------------------------------------------------------
// set-token
// ---------------------------------------------------------------------------

fn cmd_set_token(project_root: &Path, profile_override: Option<&str>, args: &SetTokenArgs) -> Result<()> {
    let cfg = load_config(project_root)?;
    let prof = resolve_profile(project_root, &cfg, profile_override)?;

    let data = PlayerData {
        service_username: args.username.clone(),
        service_token: args.token.clone(),
    };
    profile::save_player_data(&prof, &data).context(CliError::Profile)?;
    eprintln!("Token saved for '{}' in profile '{}'.", args.username, prof.name);
    Ok(())
}

// ---------------------------------------------------------------------------
// fetch
// ---------------------------------------------------------------------------

async fn cmd_fetch(project_root: &Path, profile_override: Option<&str>, args: &FetchArgs) -> Result<()> {
    let cfg = load_config(project_root)?;
    let prof = resolve_profile(project_root, &cfg, profile_override)?;
    let token = profile::load_player_data(&prof).context(CliError::Profile)?;

    let client = ModPortalClient::new().context(CliError::Api)?;
    let cache = downloader::cache_dir(project_root);
    let requests: Vec<ModRequest> = args.mods.iter().map(|n| ModRequest::latest(n)).collect();

    eprintln!("Resolving {} mod(s)...", requests.len());
    let resolved = downloader::resolve(&client, &requests).await.context(CliError::Downloader)?;
    eprintln!("Resolved {} mod(s) (including dependencies).", resolved.len());
    for m in &resolved {
        eprintln!("  {} {}", m.name, m.version);
    }

    let mods_dir = prof.mods_dir();
    eprintln!("Downloading to {}...", mods_dir.display());
    let paths = downloader::download(&client, &token, &resolved, &mods_dir, Some(&cache))
        .await
        .context(CliError::Downloader)?;
    eprintln!("Downloaded {} file(s).", paths.len());

    let mut modlist = downloader::read_modlist(&mods_dir).context(CliError::Downloader)?;
    downloader::merge_into_modlist(&mut modlist, &resolved);
    downloader::write_modlist(&mods_dir, &modlist).context(CliError::Downloader)?;

    let mut records = downloader::read_mod_records(&prof.root).context(CliError::Downloader)?;
    downloader::merge_mod_records(&mut records, &resolved);
    downloader::write_mod_records(&prof.root, &records).context(CliError::Downloader)?;
    eprintln!("Updated modlist.json and mods.json.");

    Ok(())
}

// ---------------------------------------------------------------------------
// run / dump-data / dump-defines
// ---------------------------------------------------------------------------

fn cmd_run(project_root: &Path, profile_override: Option<&str>, args: &RunArgs) -> Result<()> {
    let cfg = load_config(project_root)?;
    let prof = resolve_profile(project_root, &cfg, profile_override)?;
    let invoker = FactorioInvoker::from_config(&cfg).context(CliError::Invoker)?;

    let save = args.save.as_ref().map(|p| {
        if p.is_absolute() { p.clone() } else { prof.saves_dir().join(p) }
    });

    let output = invoker
        .run_headless(&prof.mods_dir(), &prof.mod_settings_path(), save.as_deref(), 0)
        .context(CliError::Invoker)?;

    for w in &output.warnings { eprintln!("WARN: {w}"); }
    for e in &output.errors   { eprintln!("ERROR: {e}"); }
    if output.errors.is_empty() {
        eprintln!("Run completed.");
    } else {
        eprintln!("{} error(s) found.", output.errors.len());
        std::process::exit(1);
    }
    Ok(())
}

fn cmd_dump_data(project_root: &Path, profile_override: Option<&str>) -> Result<()> {
    let cfg = load_config(project_root)?;
    let prof = resolve_profile(project_root, &cfg, profile_override)?;
    let invoker = FactorioInvoker::from_config(&cfg).context(CliError::Invoker)?;
    let path = invoker.dump_data(&prof.mods_dir(), &prof.mod_settings_path()).context(CliError::Invoker)?;
    eprintln!("data-raw-dump.json written to {}", path.display());
    Ok(())
}

fn cmd_dump_defines(project_root: &Path, profile_override: Option<&str>) -> Result<()> {
    let cfg = load_config(project_root)?;
    let prof = resolve_profile(project_root, &cfg, profile_override)?;
    let invoker = FactorioInvoker::from_config(&cfg).context(CliError::Invoker)?;
    let path = invoker.dump_defines(&prof.mods_dir(), &prof.mod_settings_path()).context(CliError::Invoker)?;
    eprintln!("defines.json written to {}", path.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn ensure_gitignore(project_root: &Path, entry: &str) -> std::io::Result<()> {
    let path = project_root.join(".gitignore");
    let existing = if path.exists() { std::fs::read_to_string(&path)? } else { String::new() };
    if existing.lines().any(|l| l.trim() == entry) {
        return Ok(());
    }
    let append = if existing.ends_with('\n') || existing.is_empty() {
        format!("{entry}\n")
    } else {
        format!("\n{entry}\n")
    };
    std::fs::write(&path, existing + &append)
}
