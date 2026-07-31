use std::collections::HashSet;
use std::io::BufRead;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use regex::Regex;

static TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?<![0-9a-f])[0-9a-f]{30}(?![0-9a-f])").expect("valid regex"));

pub type Result<T> = rootcause::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io,
    Git(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io => write!(f, "I/O error"),
            Error::Git(msg) => write!(f, "git error: {msg}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TokenMatch {
    pub file: String,
    pub line: usize,
}

/// Scan a `BufRead` source line-by-line for 30-char lowercase hex tokens.
/// Returns all matches as `(line_number, line_content)`.
///
/// Pattern matches exactly 30 consecutive `[0-9a-f]` chars not bordered by
/// further hex chars, so 40-char SHA1 hashes and other longer runs are skipped.
pub fn scan_reader(reader: impl BufRead, source_name: &str) -> Vec<TokenMatch> {
    let re = &*TOKEN_RE;
    let mut matches = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let Ok(line) = line else { break };
        if re.is_match(&line) {
            matches.push(TokenMatch {
                file: source_name.to_owned(),
                line: idx + 1,
            });
        }
    }
    matches
}

/// Scan all git-staged blobs for token matches.
/// Runs `git diff --cached --name-only` then `git show :<file>` for each file.
pub fn scan_staged(project_root: &Path) -> Result<Vec<TokenMatch>> {
    let names_output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(project_root)
        .output()
        .map_err(|_| rootcause::report!(Error::Io))?;

    if !names_output.status.success() {
        let msg = String::from_utf8_lossy(&names_output.stderr).into_owned();
        return Err(rootcause::report!(Error::Git(msg)));
    }

    let file_list = String::from_utf8_lossy(&names_output.stdout);
    let mut all_matches = Vec::new();

    for file in file_list.lines() {
        if file.is_empty() {
            continue;
        }
        let blob = Command::new("git")
            .args(["show", &format!(":{file}")])
            .current_dir(project_root)
            .output()
            .map_err(|_| rootcause::report!(Error::Io))?;

        // Non-zero exit for binary files or deleted files — skip silently.
        if !blob.status.success() {
            continue;
        }

        let reader = std::io::BufReader::new(blob.stdout.as_slice());
        let matches = scan_reader(reader, file);
        all_matches.extend(matches);
    }

    Ok(all_matches)
}

/// Scan a file on disk.
pub fn scan_file(path: &Path) -> Result<Vec<TokenMatch>> {
    let f = std::fs::File::open(path).map_err(|_| rootcause::report!(Error::Io))?;
    let reader = std::io::BufReader::new(f);
    let name = path.display().to_string();
    Ok(scan_reader(reader, &name))
}
