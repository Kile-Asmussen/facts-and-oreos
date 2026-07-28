use std::path::PathBuf;

use facts_and_oreos::{
    downloader::{self, ModRequest},
    invoker::FactorioInvoker,
};
use mod_api::{credentials, portal::ModPortalClient};
use rootcause::prelude::ResultExt as _;

type Result<T> = rootcause::Result<T, CliError>;

#[derive(Debug)]
enum CliError {
    MissingArg(&'static str),
    Credentials,
    Api,
    Downloader,
    Invoker,
    Io,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingArg(msg) => write!(f, "{msg}"),
            CliError::Credentials => write!(f, "credential error"),
            CliError::Api => write!(f, "mod portal API error"),
            CliError::Downloader => write!(f, "downloader error"),
            CliError::Invoker => write!(f, "Factorio invoker error"),
            CliError::Io => write!(f, "I/O error"),
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: facts-and-oreos <command> [args...]");
        eprintln!("Commands:");
        eprintln!("  fetch <mod-name>...   Resolve and download mods into .factorio/mods/");
        eprintln!("  login <username>      Store mod portal credentials");
        eprintln!("  run [save-file]       Run Factorio headless (bundled scenario or save file)");
        eprintln!("  dump-data             Dump data.raw as JSON via --dump-data");
        eprintln!("  dump-defines          Dump defines table via on_init hook");
        std::process::exit(1);
    }

    let result = match args[1].as_str() {
        "fetch"        => cmd_fetch(&args[2..]).await,
        "login"        => cmd_login(&args[2..]).await,
        "run"          => cmd_run(&args[2..]),
        "dump-data"    => cmd_dump_data(),
        "dump-defines" => cmd_dump_defines(),
        cmd => {
            eprintln!("Unknown command: {cmd}");
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn cmd_fetch(mod_names: &[String]) -> Result<()> {
    if mod_names.is_empty() {
        return Err(rootcause::report!(CliError::MissingArg(
            "fetch: specify at least one mod name"
        )));
    }

    let token = credentials::load()
        .context(CliError::Credentials)?
        .ok_or_else(|| {
            rootcause::report!(CliError::MissingArg(
                "No credentials found. Run: facts-and-oreos login <username>"
            ))
        })?;

    let client = ModPortalClient::new().context(CliError::Api)?;
    let project_root = PathBuf::from(".");
    let mods_dir = downloader::mods_dir(&project_root);

    let requests: Vec<ModRequest> = mod_names.iter().map(|n| ModRequest::latest(n)).collect();

    eprintln!("Resolving {} mod(s)...", requests.len());
    let resolved = downloader::resolve(&client, &requests)
        .await
        .context(CliError::Downloader)?;
    eprintln!(
        "Resolved {} mod(s) (including dependencies).",
        resolved.len()
    );
    for m in &resolved {
        eprintln!("  {} {}", m.name, m.version);
    }

    eprintln!("Downloading to {}...", mods_dir.display());
    let paths = downloader::download(&client, &token, &resolved, &mods_dir)
        .await
        .context(CliError::Downloader)?;
    eprintln!("Downloaded {} file(s).", paths.len());

    let mut modlist = downloader::read_modlist(&mods_dir).context(CliError::Downloader)?;
    downloader::merge_into_modlist(&mut modlist, &resolved);
    downloader::write_modlist(&mods_dir, &modlist).context(CliError::Downloader)?;
    eprintln!("Updated modlist.json.");

    Ok(())
}

async fn cmd_login(args: &[String]) -> Result<()> {
    let username = args
        .first()
        .ok_or_else(|| rootcause::report!(CliError::MissingArg("login: specify username")))?;
    let password = read_password(username).context(CliError::Io)?;

    let client = ModPortalClient::new().context(CliError::Api)?;
    let token = client
        .login(username, &password)
        .await
        .context(CliError::Api)?;
    credentials::save(&token).context(CliError::Credentials)?;
    eprintln!(
        "Logged in as {}. Credentials saved.",
        token.service_username
    );
    Ok(())
}

fn cmd_run(args: &[String]) -> Result<()> {
    let project_root = PathBuf::from(".");
    let mods_dir = downloader::mods_dir(&project_root);
    let invoker = FactorioInvoker::from_env(&project_root).context(CliError::Invoker)?;
    let save = args.first().map(PathBuf::from);
    let output = invoker
        .run_headless(&mods_dir, save.as_deref(), 0)
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

fn cmd_dump_data() -> Result<()> {
    let project_root = PathBuf::from(".");
    let mods_dir = downloader::mods_dir(&project_root);
    let invoker = FactorioInvoker::from_env(&project_root).context(CliError::Invoker)?;
    let path = invoker.dump_data(&mods_dir).context(CliError::Invoker)?;
    eprintln!("data-raw-dump.json written to {}", path.display());
    Ok(())
}

fn cmd_dump_defines() -> Result<()> {
    let project_root = PathBuf::from(".");
    let mods_dir = downloader::mods_dir(&project_root);
    let invoker = FactorioInvoker::from_env(&project_root).context(CliError::Invoker)?;
    let path = invoker.dump_defines(&mods_dir).context(CliError::Invoker)?;
    eprintln!("defines.json written to {}", path.display());
    Ok(())
}

fn read_password(username: &str) -> std::io::Result<String> {
    eprint!("Password for {username}: ");
    let mut password = String::new();
    #[cfg(unix)]
    {
        let original = get_termios();
        let mut silent = original;
        silent.c_lflag &= !libc::ECHO;
        set_termios(&silent);
        std::io::stdin().read_line(&mut password)?;
        set_termios(&original);
        eprintln!();
    }
    #[cfg(not(unix))]
    {
        std::io::stdin().read_line(&mut password)?;
    }
    Ok(password.trim_end_matches('\n').to_string())
}

#[cfg(unix)]
fn get_termios() -> libc::termios {
    unsafe {
        let mut t = std::mem::zeroed();
        libc::tcgetattr(0, &mut t);
        t
    }
}

#[cfg(unix)]
fn set_termios(t: &libc::termios) {
    unsafe { libc::tcsetattr(0, libc::TCSANOW, t) };
}
