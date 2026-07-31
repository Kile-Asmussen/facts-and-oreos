# Subgoal CC: Mod Profiles

## Decisions (resolved)

1. Bundled helper mod: embed all files from `facts-and-oreos-helper-mod/` into the
   binary at compile time (via `include_str!` / `include_bytes!` in build.rs or a
   dedicated `helper-mod` crate), then extract/copy into each new profile's mods dir.
   Source of truth is `./facts-and-oreos-helper-mod/` in the repo.

2. Profile creation is always explicit (`profile new <name>`). Using `--profile <name>`
   for a non-existent profile is an error with a hint to run `profile new`.

3. An `init` command bootstraps the project: creates `.facts-and-oreos.toml`, creates
   the default profile, and extracts the helper mod into it. All other commands fail
   immediately if `.facts-and-oreos.toml` is absent (no implicit fallback).

4. Add `toml` crate for config parsing/writing.

---

## Central config file

`.facts-and-oreos.toml` lives at the project root (checked in).
`.facts-and-oreos.local.toml` is an optional local override (gitignored), merged on top.
Both use the same schema; local overrides win per-key.

Schema:
```toml
factorio-bin = "/path/to/factorio"   # required for invoke commands
active-profile = "default"           # name of the currently-active profile

# Future keys (placeholders, not implemented in CC):
# log-level = "info"
# pipeline-lua-version = "5.2"
```

Merge behaviour: load base `.facts-and-oreos.toml`, then if `.facts-and-oreos.local.toml`
exists, overwrite any present keys with its values (simple field-level merge, no deep
merging needed given the flat schema).

`ProjectConfig` struct in `src/config.rs`:
```rust
pub struct ProjectConfig {
    pub factorio_bin: Option<PathBuf>,
    pub active_profile: String,       // default: "default"
}
```

All commands call `ProjectConfig::load(project_root)` first; if `.facts-and-oreos.toml`
does not exist, return a clear error:
  "No .facts-and-oreos.toml found. Run `facts-and-oreos init` to initialise the project."

---

## Directory layout

```
.facts-and-oreos.toml          ← checked in config (factorio-bin, active-profile, …)
.facts-and-oreos.local.toml    ← gitignored local overrides (e.g. factorio-bin path)
.profiles/
  default/
    mods/
      modlist.json
      mod-settings.dat
      facts-and-oreos-helper-mod/   ← extracted from binary at `profile new` time
      SomeMod_1.2.3.zip
      …
    saves/
  other-profile/
    mods/
    saves/
```

`.factorio/` is now purely a reference/vanilla directory (no user-data mods).

---

## Active profile resolution

Precedence (highest first):
1. `--profile <name>` CLI flag
2. `FAO_PROFILE` environment variable
3. `active-profile` in merged config
4. (no fallback — config is required to exist)

`Profile` struct exposes: `mods_dir()`, `saves_dir()`, `mod_settings_path()`, `modlist_path()`.

---

## Helper mod embedding

The files under `facts-and-oreos-helper-mod/` are embedded at compile time.
Implementation approach: a `helper-mod` sub-crate (or inline in `src/helper_mod.rs`)
that uses `include_str!` / `include_bytes!` for each known file, exposing:

```rust
pub struct HelperModFile { pub rel_path: &'static str, pub contents: &'static [u8] }
pub fn all_files() -> &'static [HelperModFile] { … }
```

`Profile::install_helper_mod(&self)` iterates `all_files()` and writes each file
into `self.mods_dir() / rel_path`, creating parent dirs as needed.
Called from `profile new` and `init`.

Known files to embed:
- `info.json`
- `scenarios/empty/control.lua`
- `scenarios/empty/description.json`
- `scenarios/empty/mapgen-settings.json`
- `scenarios/dump-defines/control.lua`
- `scenarios/dump-defines/description.json`
- `scenarios/dump-defines/mapgen-settings.json`

If new files are added to `facts-and-oreos-helper-mod/`, they must be added to the
embed list manually (or a build.rs glob can generate the list).

---

## TODO list

### CC0. `init` command  (`src/main.rs`, `src/config.rs`, `src/profile.rs`)

- [ ] `facts-and-oreos init`
  - Fail if `.facts-and-oreos.toml` already exists (don't clobber; suggest `--force`).
  - Prompt user for `factorio-bin` path (or accept `--factorio-bin <path>` flag).
  - Write `.facts-and-oreos.toml` with `factorio-bin` and `active-profile = "default"`.
  - Create `.profiles/default/mods/` and `.profiles/default/saves/`.
  - Extract helper mod into `.profiles/default/mods/facts-and-oreos-helper-mod/`.
  - Write minimal `mod-settings.dat` into `.profiles/default/mods/`.
  - Write minimal `modlist.json` (base mod enabled, helper mod enabled) into `.profiles/default/mods/`.
  - Print summary of what was created.
  - Append `.facts-and-oreos.local.toml` and `.profiles/*/saves/` and `.profiles/*/mods/*.zip`
    to `.gitignore` (create `.gitignore` if absent; append if already present but entry missing).

### CC1. `ProjectConfig`  (`src/config.rs`)

- [ ] `ProjectConfig { factorio_bin: Option<PathBuf>, active_profile: String }`
- [ ] `ProjectConfig::load(project_root: &Path) -> Result<Self>`
  - Error (with hint) if `.facts-and-oreos.toml` is absent.
  - Parse base TOML, then merge `.facts-and-oreos.local.toml` if present.
- [ ] `ProjectConfig::write(project_root: &Path, cfg: &Self) -> Result<()>`
  - Writes only `.facts-and-oreos.toml` (never the local file).
- [ ] Export from `lib.rs`.
- [ ] Add `toml` crate dependency to `Cargo.toml`.

### CC2. `Profile` struct  (`src/profile.rs`)

- [ ] `Profile { root: PathBuf }` with accessors:
  `mods_dir()`, `saves_dir()`, `mod_settings_path()`, `modlist_path()`
- [ ] `Profile::resolve(project_root, cfg: &ProjectConfig, explicit: Option<&str>) -> Result<Profile>`
  - Applies precedence: explicit flag → FAO_PROFILE env → cfg.active_profile.
  - Errors if resolved profile dir does not exist (with hint to run `profile new`).
- [ ] `Profile::ensure_dirs(&self) -> io::Result<()>`
  - Creates `mods/` and `saves/` inside `root`.
- [ ] `Profile::install_helper_mod(&self) -> io::Result<()>`
  - Writes all `helper_mod::all_files()` into `self.mods_dir() / facts-and-oreos-helper-mod/`.
  - Overwrites existing files (idempotent re-install).
- [ ] Export from `lib.rs`.

### CC3. Helper mod embedding  (`src/helper_mod.rs`)

- [ ] `HelperModFile { rel_path: &'static str, contents: &'static [u8] }`
- [ ] `all_files() -> &'static [HelperModFile]`
  - Uses `include_bytes!` for each file listed in "Helper mod embedding" section above.
  - Paths must match `facts-and-oreos-helper-mod/<rel_path>` relative to workspace root.
- [ ] Export from `lib.rs`.

### CC4. Thread config + profile through all commands  (`src/main.rs`)

- [ ] All commands (except `init`) call `ProjectConfig::load(".")` first; propagate error.
- [ ] Add global `--profile <name>` flag parsed before subcommand; also check `FAO_PROFILE` env.
- [ ] Pass resolved `Profile` into every cmd_* function.
- [ ] `cmd_fetch`: `profile.mods_dir()` replaces `downloader::mods_dir(&project_root)`.
- [ ] `cmd_run`: `profile.mods_dir()` and `profile.saves_dir()`. Save-file arg is relative
      to `profile.saves_dir()` if not absolute.
- [ ] `cmd_dump_data`, `cmd_dump_defines`: `profile.mods_dir()`.
- [ ] `FactorioInvoker::from_env` replaced by `FactorioInvoker::from_config(cfg)` using
      `cfg.factorio_bin` (falls back to `FACTORIO_BIN` env for compatibility).
- [ ] Update help text.

### CC5. `FactorioInvoker` config integration  (`src/invoker.rs`)

- [ ] Add `FactorioInvoker::from_config(cfg: &ProjectConfig) -> Result<Self>`.
  - Uses `cfg.factorio_bin` if set, then `FACTORIO_BIN` env, then errors.
- [ ] `make_isolated_env`: accept `mod_settings_src: &Path` instead of generating fresh.
  - Caller passes `profile.mod_settings_path()`.
- [ ] All three public methods (`run_headless`, `dump_data`, `dump_defines`) take
      `mods_dir: &Path` and `mod_settings_src: &Path`.

### CC6. `mod-settings.dat` per profile  (`src/profile.rs`, `src/mod_settings.rs`)

- [ ] `Profile::init_mod_settings(&self) -> io::Result<()>`
  - If `self.mod_settings_path()` does not exist, write a minimal `ModSettings::empty(…)`.
  - Called by `profile new` and `init`.
- [ ] `make_isolated_env` (CC5) copies the profile's `mod-settings.dat` into the temp dir
      rather than generating a fresh one.

### CC7. `profile` management commands  (`src/main.rs`)

- [ ] `facts-and-oreos profile list`
  - Enumerate `.profiles/` subdirectories; mark active profile with `*`.
- [ ] `facts-and-oreos profile new <name>`
  - Fail if `.profiles/<name>/` already exists.
  - `ensure_dirs`, `install_helper_mod`, `init_mod_settings`, write minimal `modlist.json`.
- [ ] `facts-and-oreos profile set-default <name>`
  - Verify profile exists; write `active-profile = "<name>"` to `.facts-and-oreos.toml`.
- [ ] `facts-and-oreos profile delete <name>`
  - Refuse to delete active profile.
  - Require `--force` if profile mods dir is non-empty (list mods before refusing).
- [ ] `facts-and-oreos profile clone <src> <dst>`
  - Copy `mods/*.zip` + `modlist.json`; re-init `mod-settings.dat`; do NOT copy saves.

### CC8. `.gitignore` management  (called from `init`)

- [ ] Helper `fn ensure_gitignore_entry(project_root, line: &str)`
  - Creates `.gitignore` if absent; appends `line` only if not already present.
- [ ] `init` adds: `.facts-and-oreos.local.toml`, `.profiles/*/saves/`, `.profiles/*/mods/*.zip`.

### CC9. Migration command  (`src/main.rs`)

- [ ] `facts-and-oreos profile migrate [--profile <name>]`
  - Detect `.factorio/mods/*.zip` and/or non-trivial `modlist.json`.
  - Move them to the target profile's `mods/` dir.
  - Do NOT move `facts-and-oreos-helper-mod/` (already embedded).
  - Print what was moved.
- [ ] On startup of any non-init command: if `.factorio/mods/` has stray `.zip` files,
      print one-time hint suggesting `profile migrate`.

### CC10. Remove `downloader::mods_dir`

- [ ] Delete `pub fn mods_dir(project_root: &Path)` from `downloader.rs`.
  - Callers all use `profile.mods_dir()` after CC4.

---

## Non-goals for CC

- Per-profile Factorio binary (binary is global in config).
- Profile sharing / remote sync.
- Deep TOML merging (flat key overrides are sufficient).
- Profiles for pipeline-sim (I/J/K) — they inherit `Profile` naturally.
