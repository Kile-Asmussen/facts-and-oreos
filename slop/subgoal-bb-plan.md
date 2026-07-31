# Subgoal BB: Mod Downloader Overhaul

## Goals

1. Shared zip cache so repeated `fetch` across profiles never re-downloads the same file.
2. Credential source changes: read `player-data.json` from the profile directory instead
   of from `~/.factorio/`. The `login` command writes to the profile's player-data.json.
3. Remove `downloader::mods_dir` (the leftover `.factorio/mods/` helper from before CC).
4. Per-profile mod tracking file (`mods.json`) so `mods/` dirs can be fully gitignored.
5. `check-token` CLI subcommand that scans for 30-char lowercase hex strings.
6. Git pre-commit hook installed by `init --git` that calls `facts-and-oreos check-token`.

---

## Zip cache

Downloaded zip files are large and identical regardless of which profile uses them.
A shared cache avoids re-downloading when a mod is added to a second profile.

Cache location: `.cache/mods/` at the project root (gitignored, not profile-specific).

Download flow:
1. Check profile's mods dir for `file_name` + valid SHA1 → use it directly (already there).
2. Check cache (`<project_root>/.cache/mods/<file_name>`) for valid SHA1 → copy into profile.
3. If neither, download from portal → write to cache → copy into profile.

Corrupt/partial cache entries are deleted and re-fetched.
`download()` gains `cache_dir: Option<&Path>`; callers pass `Some(cache_dir(project_root))`.

---

## player-data.json per profile

Per-profile at `.profiles/<name>/player-data.json` (gitignored, fishing-denied).
`login` writes it, `fetch` reads it. Existing mod-api `credentials` module left intact.

---

## Per-profile mod tracking: mods.json

The profile's `mods/` directory contains zip files and the extracted helper mod.
Both are large/generated and should be gitignored. To allow a profile to be reproduced
from just the tracking file, we maintain `.profiles/<name>/mods.json`:

```json
{
  "mods": [
    { "name": "SomeMod", "version": "1.2.3", "file_name": "SomeMod_1.2.3.zip", "sha1": "abc..." },
    ...
  ]
}
```

- Written/updated by `fetch` after every successful download.
- Read by a future `restore` command (not in BB scope) to re-populate mods/ from cache or portal.
- `modlist.json` (already tracked) records enabled/disabled state; `mods.json` records what is
  physically present. They are complementary.
- `mods.json` is committed; `mods/` (except the tracking files) is gitignored.

Gitignore entries managed by `init --git`:
```
.profiles/*/mods/
```
(replaces the earlier finer-grained `.profiles/*/mods/*.zip` entry)

---

## check-token subcommand

`facts-and-oreos check-token [--staged] [file...]`

Scans files (or git staged blobs when `--staged`) for the Factorio service token pattern:
exactly 30 consecutive lowercase hex characters not surrounded by further hex chars.

Pattern: `(?<![0-9a-f])[0-9a-f]{30}(?![0-9a-f])`

- Without arguments and without `--staged`: reads stdin.
- With `--staged`: calls `git diff --cached --name-only`, then reads each staged blob
  via `git show :<file>` piped through the scanner.
- With file arguments: reads those files.
- Exits 0 if clean, 1 if a match is found (prints file + line number).
- Implemented in `src/check_token.rs`, exported from `lib.rs`, invoked from `main.rs`.

Using `grep -P` from a shell hook is fragile (PCRE not always available, shell subshell
exit-code propagation issues). Delegating to the compiled binary is more robust.

---

## Git hook installation (init --git)

`facts-and-oreos init --git` (can be combined with other init flags):

1. Runs `git init` if `.git/` does not exist (creates the repo).
2. Determines hooks dir: reads `git config core.hooksPath`; if unset, defaults to
   `<git-dir>/hooks/` which is `.git/hooks/` for a standard repo.
   Since we know the project uses the default, the tool hardcodes `.git/hooks/` but
   reads `core.hooksPath` via `std::process::Command` to override if configured.
3. Writes `.git/hooks/pre-commit` (or the configured hooks dir) containing:
   ```sh
   #!/bin/sh
   facts-and-oreos check-token --staged
   ```
4. Sets the hook executable (`chmod +x` via `std::fs::Permissions` on Unix).
5. Adds gitignore entries: `.profiles/*/mods/`, `.cache/`, `.facts-and-oreos.local.toml`,
   `**/player-data.json` (idempotent via existing `ensure_gitignore` helper).

The hook calls `facts-and-oreos` by name, so it must be on `$PATH`. Document this
requirement in the hook file as a comment.

---

## Implementation decisions (recorded post-implementation)

### check-token as a binary subcommand, not a shell grep
The hook is a one-liner calling `facts-and-oreos check-token --staged`. This gives
portable regex (via the `regex` crate), structured output, and the same scanner
available interactively. No PCRE dependency, no subshell exit-code fragility.

### scan_staged uses git commands via std::process::Command
`git diff --cached --name-only` enumerates staged files; `git show :<file>` reads each
blob. Non-zero exits (binary files, deletions) are skipped silently. This avoids any
temp-file management and works correctly for renames (git show uses the index path).

### mods.json lives at profile root, not inside mods/
`.profiles/<name>/mods.json` sits alongside `saves/`, one level above `mods/`.
This means the whole `mods/` directory can be gitignored, while `mods.json` (a small
JSON manifest) is committed. The manifest records name, version, file_name, sha1 for
each installed zip so a future `restore` command can reproduce the mods/ dir from
cache or portal without any manual tracking.

### .profiles/*/mods/ gitignored wholesale (replaces *.zip)
The old pattern `.profiles/*/mods/*.zip` was replaced with `.profiles/*/mods/` entirely.
The helper mod (a directory, not a zip) is also regenerated at `profile new`/`init` time
from the embedded binary data, so gitignoring the whole mods dir is safe. modlist.json
and mod-settings.dat, which were previously inside mods/, are still inside mods/ — they
are regenerated or profile-managed and need not be committed.

### download() cache flow: mods_dir → cache → download → cache → mods_dir
The three-step check (profile mods dir, shared cache, network) means:
1. A mod already present with a valid SHA1 in the profile is never touched.
2. A mod in the cache (from any profile's previous fetch) is copied, not downloaded.
3. A fresh download goes to cache first, then is copied to the profile's mods dir.
Corrupt cache entries (SHA1 mismatch) are deleted before re-fetching.
The `cache_dir: Option<&Path>` parameter allows callers (e.g. tests) to opt out of caching.

### init --git reads core.hooksPath at runtime
Rather than hardcoding `.git/hooks/`, `init --git` runs `git config core.hooksPath`
via Command and uses the output if non-empty, falling back to `.git/hooks/`. This
respects monorepo setups and custom hook managers (e.g. lefthook, husky) that redirect
the hooks directory.

### credentials::load/save in mod-api kept but no longer used by CLI
The original `credentials` module (writing to `~/.factorio/player-data.json`) remains
in mod-api for library consumers. The CLI exclusively uses `profile::load_player_data`
and `profile::save_player_data` which write to `.profiles/<name>/player-data.json`.
No migration path is provided — users re-login after upgrading.

### download_mod_by_url takes dest as full path, not directory
`download_mod_by_url(url, file_name, token, dest: &Path)` writes directly to `dest`
(the full file path, not a directory). The `file_name` parameter is kept for future
use (logging, validation) but is not used to construct the path — the caller is
responsible for joining `cache_dir / file_name` before calling.

## TODO list

### BB1. Zip cache in `src/downloader.rs` ✓

- [x] Add `cache_dir: Option<&Path>` parameter to `download()`.
- [x] Before downloading, check cache for `file_name` with valid SHA1 → `fs::copy` into mods_dir.
- [x] After successful download → `fs::copy` into cache (create dir if needed).
- [x] Corrupt/missing cache entry: delete and re-fetch.
- [x] `pub fn cache_dir(project_root: &Path) -> PathBuf` → `project_root/.cache/mods/`.
- [x] Delete `pub fn mods_dir(project_root: &Path)`.

### BB2. Per-profile mod tracking (`src/downloader.rs`, `src/profile.rs`) ✓

- [x] `ModRecord { name, version, file_name, sha1 }` with serde derive.
- [x] `read_mod_records(profile_root)` / `write_mod_records` / `merge_mod_records`.
- [x] `mod_records_path()` accessor on `Profile` → `self.root / "mods.json"`.
- [x] `download()` keeps `Vec<PathBuf>` return; caller zips with `resolved` slice.

### BB3. player-data.json per profile (`src/profile.rs`) ✓

- [x] `player_data_path()` accessor on `Profile`.
- [x] `load_player_data` / `save_player_data` free functions; 0o600 on Unix.

### BB4. Wire BB1+BB2+BB3 through CLI (`src/main.rs`) ✓

- [x] `cmd_login` now takes profile arg, writes to profile's `player-data.json`.
- [x] `cmd_fetch` reads from profile's `player-data.json`; passes cache to `download()`;
      updates `mods.json` after download.
- [x] `cmd_init` adds `.cache/` and all other gitignore entries.

### BB5. `check-token` subcommand (`src/check_token.rs`, `src/main.rs`) ✓

- [x] `scan_reader`, `scan_staged`, `scan_file`; `TokenMatch { file, line, snippet }`.
- [x] `regex` crate; pattern `(?<![0-9a-f])[0-9a-f]{30}(?![0-9a-f])`.
- [x] CLI: `check-token [--staged] [file...]`; exits 1 on match, 0 on clean.
- [x] Exported from `lib.rs`.

### BB6. Git hook installation (`src/main.rs`) ✓

- [x] `init --git`: `git init` if `.git/` absent; reads `core.hooksPath` via Command;
      writes `pre-commit` calling `facts-and-oreos check-token --staged`; chmod 0o755.

### BB7. `download_mod_by_url` on `ModPortalClient` (`mod-api/src/portal.rs`) ✓

- [x] Takes `url`, `file_name` (for future use), `token`, `dest: &Path` (full path).
- [x] `downloader::download` uses this instead of `download_mod`.
- [x] `download_mod` kept for external callers.

---

## Gitignore additions managed by `init --git`

New (replaces `.profiles/*/mods/*.zip`):
- `.profiles/*/mods/`   ← whole mods dir gitignored; mods.json lives one level up

Already handled by `init` (no change):
- `.facts-and-oreos.local.toml`
- `.cache/`
- `**/player-data.json`
- `.profiles/*/saves/`

---

## Security notes

- `check-token` is the canonical scanner; the hook is a thin shell wrapper around it.
- The regex uses negative lookahead/lookbehind to avoid matching substrings of SHA1
  hashes (40 chars) or other longer hex strings.
- False positives accepted; `git commit --no-verify` is the documented escape hatch.
- `PlayerData` has no `Debug` impl — unchanged.
