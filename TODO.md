# Goal: Replicate factorio mod loading pipeline

The purpose is to provide a fast sanity check system for factorio mod development.

The structure of the project will be a core rust library containing several binaries.
Consider design options of each subgoal in dialogue with user and fill out a comprehensive to-do list of each subgoal.

## Subgoal A: Factorio mod api integration

Implemented in `mod-api/` crate. Reimplemented from scratch rather than depending on the aging `factorio-mod-api` crate.

- [x] A1. Dependencies: `rootcause`, `reqwest` (json+stream+query), `serde`+`serde_json`, `semver`, `tokio`
- [x] A2. `ModInfo` — full local `info.json` struct (name, version, title, author, contact, homepage, description, factorio_version, dependencies)
- [x] A3. `ModDependency` parser — all five flavors (normal, !, ?, (?), ~) with optional semver comparator; roundtrip Display
- [x] A4. `modlist.json` types — `ModListEntry { name, enabled }`; `ModSpec` / `ModRelease` for portal responses
- [x] A5. `ModPortalClient` — `get_mod_spec`, `login`, `download_mod` against mods.factorio.com / auth.factorio.com
- [x] A6. Credential store (`credentials` module) — load/save `ApiToken` to XDG config dir at mode 0o600; no `Debug` impl on `ApiToken`

NOTE: `ApiToken` has no `Debug` impl to prevent accidental logging. Credentials are now stored per-profile at `.profiles/<name>/player-data.json` (BB3); the XDG path is no longer used by the CLI.

## Subgoal B: Implement Factorio mod downloader

Make a tool that takes a set of mods referenced by name and optionally version, download all mods and their prerequisites (available by inspecting their info.json files), unpack them to a suitable caching directory.

This should include a utility for updating a modlist.json file, same as the game.

Implemented in `src/downloader.rs` as a library; exposed via CLI in `src/main.rs`.

- [x] B1. Mod version resolver: greedy latest-compatible, walks transitive required deps, checks incompatibility constraints
- [x] B2. Download pipeline: checks cache (by filename + sha1) before downloading; verifies sha1 after download
- [x] B3. Mods directory: `.factorio/mods/` relative to project root (`Name_Version.zip` layout); project-local, not XDG
- [x] B4. `modlist.json` read/write: merge resolved mods in, enable them, preserve existing user entries
- [x] B5. CLI: `facts-and-oreos fetch <mod>...` and `facts-and-oreos login <username>` (password read with echo-off)

## Subgoal C: A suite of tools to invoke factorio in headless mode

Implemented in `src/invoker.rs` + `src/mod_settings.rs`. Uses a fully isolated temp dir with a generated `config.ini` and `mod-settings.dat` so the user's real Factorio installation is never touched.

- [x] C1. `FactorioInvoker`: binary from `FACTORIO_BIN` env; `read-data` auto-detected from binary path
- [x] C2. Config generation: writes isolated `config.ini` (read-data → Factorio install, write-data → temp dir)
- [x] C3. `mod-settings.dat` codec: full PropertyTree binary format, bit-identical roundtrip verified against reference file; generates minimal empty settings for each run
- [x] C4. Helper mod (`facts-and-oreos-helper-mod/` in repo): `info.json`, `scenarios/empty/`, `scenarios/dump-defines/`; embedded in binary via CC3 and extracted into each profile's mods dir
- [x] C5. `run_headless(save: Option<&Path>, mods_dir, until_tick)`: `--load-scenario facts-and-oreos/empty` or `--load-game <path>`; returns structured `RunOutput` with parsed errors/warnings
- [x] C6. `dump_data(mods_dir, mod_settings_src)`: `--dump-data`; moves result to `<profile>/mods/data-raw-dump.json`
- [x] C7. `dump_defines(mods_dir, mod_settings_src)`: runs dump-defines scenario; moves result to `<profile>/mods/defines.json`
- [x] C8. CLI: `run [save]`, `dump-data`, `dump-defines`

NOTE: `find_binary_from_project` is removed; binary is now configured via `factorio-bin` in `.facts-and-oreos.toml` or the `FACTORIO_BIN` env var (CC1/CC5). `from_env` kept on `FactorioInvoker` for test use only; `from_config` is the runtime path.

NOTE: To speed up scenario loading, it may be beneficial to supply custom map generation settings (via `--map-gen-settings`) that produce the most minimal possible world — e.g. no resources, no enemies, smallest map size. This would reduce world generation time significantly for the empty and dump-defines scenarios.

NOTE: Consider adding a CLI command to launch Factorio for ordinary (non-headless) play against the project's mod set, to aid the mod developer in testing their work interactively without needing to manage Factorio's own mod directory.

NOTE: Investigate using Factorio's replay system as a basis for automated testing in mod development workflows — a replay encodes all player actions deterministically, which could allow replaying against modified mod code and checking for divergence or errors.

## Subgoal BB: Mod Downloader Overhaul

Refines the downloader after the profile system (CC) is in place.

Full design in `./slop/subgoal-bb-plan.md`.

- [x] BB1. Shared zip cache at `.cache/mods/`; `download()` gains `cache_dir: Option<&Path>`; delete `downloader::mods_dir`
- [x] BB2. Per-profile `mods.json` tracking file (`ModRecord`; read/write/merge); `mods/` dirs fully gitignored
- [x] BB3. `player-data.json` per profile: `player_data_path()`, `load_player_data`, `save_player_data` in `profile.rs`
- [x] BB4. Wire BB1+BB2+BB3 through CLI (`login`/`fetch` use profile credentials; `fetch` updates `mods.json`; `init` adds `.cache/` to gitignore)
- [x] BB5. `check-token` subcommand (`src/check_token.rs`): scans stdin/files/staged blobs for 30-char lowercase hex; exit 1 on match
- [x] BB6. `init --git`: runs `git init` if needed, reads `core.hooksPath`, writes pre-commit hook calling `facts-and-oreos check-token --staged`
- [x] BB7. `download_mod_by_url` on `ModPortalClient` to avoid redundant `get_mod_spec` call during download

## Subgoal CC: Mod Profiles

Cross-cutting feature: named, self-contained profiles under `.profiles/<name>/`, each
holding its own mods, modlist, mod-settings.dat, and saves. A central
`.facts-and-oreos.toml` config file (with optional `.facts-and-oreos.local.toml` local
override) replaces `.factorio/project.toml`. All commands require the config to exist.

Full design in `./slop/subgoal-cc-plan.md`.

- [x] CC0. `init` command: creates config, default profile, extracts helper mod, patches `.gitignore`
- [x] CC1. `ProjectConfig` (`src/config.rs`): load/write `.facts-and-oreos.toml` + local merge; `toml` crate
- [x] CC2. `Profile` struct (`src/profile.rs`): accessors, `resolve`, `ensure_dirs`, `install_helper_mod`
- [x] CC3. Helper mod embedding (`src/helper_mod.rs`): `include_bytes!` all files from `facts-and-oreos-helper-mod/`
- [x] CC4. Thread config + profile through all CLI commands
- [x] CC5. `FactorioInvoker::from_config`; `make_isolated_env` takes `mod_settings_src: &Path`
- [x] CC6. `mod-settings.dat` per profile: init on `profile new`; copied into temp env each run
- [x] CC7. `profile list/new/set-default/delete/clone` commands
- [x] CC8. `.gitignore` management helper (called from `init`)
- [x] CC9. `profile migrate` command; stray-zip hint on startup
- [x] CC10. Delete `downloader::mods_dir` (completed as part of BB1)

## Subgoal D: Create mlua integration with Factorio Lua
 
Add Factorio Lua as an accessible implementation of lua in the mlua library. This includes some kind of compilation strategy for compiling the flua implementation. Check ./reference/flua/ for the code.

Make sure that there is compliance with the descriptions of functionality in available as plain text in ./reference/api/auxiliary/libraries.txt as well as the 'determinism' requirement posited by Factorio's design (though not quite as important for a simple checking tool). This should already be given in the flua implementation itself.

NOTE: the require function in Factorio allows mods to load data out of core/lualib as well as files local to the mod, and from other mods using __modname__.luafile

Full plan with compilation strategy analysis: ./slop/subgoal-d-plan.md

Summary of approach:
- FLua is Lua 5.2.1 + Factorio extensions (custom_pow, lua_getlfield/setlfield, lua_pushunsigned, etc.), compiled as C++
- FLua managed as a git submodule (reproducible, updatable); build.rs guards against forgotten submodule init
- NOTE: override_printf.h appears to depend on trio but is never included by any .c file -- dead code, not a build concern
- Re-implement flua-src-rs in-tree as flua-src/ crate (tiny, ~200 lines, no external dep needed)
- facts-mlua-sys/ crate: mlua-sys lua52 FFI + FLua-only extension symbols, links against flua-src output
- Use upstream mlua (crates.io) with lua52 non-vendored feature, pointed at our flua-src static lib via env vars

- [x] D1. Confirmed: override_printf.h never included by .c files; trio not a build dependency
- [x] D2. `flua-src/` crate with Build/Artifacts API; flua git submodule at `flua-src/flua/`; submodule guard in build.rs
- [x] D3. `flua-mlua-sys/` crate: lua52 FFI base + FLua extension symbols (lua_getlfield/setlfield etc.) + build.rs
- [x] D4. mlua 0.12 integration via `mlua-sys-shim` patch: shim re-exports flua-mlua-sys, version pinned to 0.11.0; three missing constants (LUA_MAX_UPVALUES, LUA_TRACEBACK_STACK, SYS_MIN_ALIGN) added to flua-mlua-sys
- [x] D5. Smoke-test: open FLua state, run 1+1, call lua_getlfield, verify tostring(2^10)
- [x] D6. Moved to Subgoal I: custom require() loader via mlua high-level API alongside pipeline setup

## Subgoal E: Replicate LuaHelpers

LuaHelpers is a library available at the mod loading stage, and so needs to be made available to the code. Check ./reference/api/classes/LuaHelpers.txt for information.

- TODO list for this subgoal goes here

## Subgoal F: Code generation for implementing Factorio's prototypes

The entirety of Factorio's Prototype documentation is available in machine readable format, and code can therefore be generated for it. ./reference/api/prototypes-api.json and ./reference/api/runtime-api.json

Of particular interest: Serde-integrated Rust type definitions (in build.rs), and Lua annotations for the EmmyLua LSP (as an invoked tool).

- TODO list for this subgoal goes here

## Subgoal G: Verification of integrity of prototypes

Using the generated prototype definitions, at the end of the simulated loading stages the protypes should be checked against the definitions. IDs should be cross referenced for consistency, numeric and string values should be checked for validity, paths to graphics and sounds should be checked for existence and file type, etc.

- TODO list for this subgoal goes here

## Subgoal H: Localisation string verification

One problem in factorio mod development is localization strings. The mod loader simulation should be able to determing which localisation strings are not defined. 

NOTE: the behavior of the localised_name and localised_description fields of prototypes exclude them from ordinary linking to the default localisation string IDs, but also that they themselves contain additional localisation strings.

NOTE: there is a defaulting behavior for naming/description of item-building-recipe triplets:

https://wiki.factorio.com/Tutorial:Localisation#Default_Behavior(s)_for_finding_an_Unspecified_Localised_String

- TODO list for this subgoal goes here

## Subgoal I: Mod loading

Once mod loading begins, several global Lua variables are defined in the factorio mod loading pipeline, which is available for inspection by each mod. The mods will have to be ordered in terms of their prerequisites, and loaded in a deterministic ordering scheme, as well as checking for cycling dependencies, etc.

NOTE: Determine how Factorio does the ordering of non-dependent mods, and replicate that if possible.

NOTE: Custom require() loader belongs here. Implement via mlua high-level API (create_function + closure capturing pipeline state, insert into package.searchers). Path conventions:
- mod-local: require("myfile") → <modname>/myfile.lua
- core lualib: require("util") → core/lualib/util.lua
- cross-mod: require("__othermod__.file") → othermod/file.lua

- TODO list for this subgoal goes here

## Subgoal J: Setting stage simulation

The factorio startup loading process is split into a settings stage and a data stage. Between the two, the lua runtime is destroyed entirely. After the settings stage, the declared settings need to be available for the user to edit for the data stage run for instance through a suitable configuration file including the localised names/descriptions within the config file for improved usability.

- TODO list for this subgoal goes here

## Subgoal H: Prototype stage simulation

This is the meat of the project, the data loading stage is where most of the checks implemented in subgoal G and H are applicable. It will however use much of the same logic as in I and J.

Furthermore this stage (and the settings stage) should implement some kind of logic for determining not only which prototypes are added with data:extend but also which prototypes are edited by each mod.

- TODO list for this subgoal goes here

## Subgoal K: Control stage simulation

In factorio, when starting a scenario or loading a saved game, the control stage executes, letting the developer install hooks. (This again requires a fresh Lua VM instance.) While it is beyond the scope of this project to simulate invoking these hooks, there should be a sanity check for it, such as for localisation strings and various pitfalls (for instance each mod can only have one on_init hook installed.)

- TODO list for this subgoal goes here

## Subgoal L: Design for a frictionless usage in mod development

The purpose of this tool is to be invoked within a development directory of a mod, and simulate the process of loading the mod into the game. Considerations need to be given to the user experience for mod developers.

This includes usage guides, sensible choice of caching directories, etc.

- TODO list for this subgoal goes here

## Subgoal M: MCP for inspecting JSON dumps of data.raw

The purpose of this MCP is to assist an LLVM in working with Factorio prototypes. The simplest solution is using JQ scripts for the investigation through the jaq rust library:

https://docs.rs/jaq-core/latest/jaq_core/

- TODO list for this subgoal goes here

## Subgoal N: Mod upload command

The Factorio mod portal supports API key-based mod uploads (distinct from the service token used for downloads). This enables automated publishing of mod zips directly from the tool.

The API key is created at https://factorio.com/profile under "API keys" and has a separate lifecycle from the service token. It should be stored separately (e.g. `api-key` field in `player-data.json` or a separate file) and supplied via `facts-and-oreos set-api-key <key>`.

A `publish` (or `upload`) subcommand would then package the mod zip and POST it to the portal upload endpoint.

- TODO list for this subgoal goes here

## Subgoal O: Claude Skill for Factorio modding

A skill giving descriptions for how to mod Factorio using this tool suite, as well as the Emmylua LSP as a plugin.

- TODO list for this subgoal goes here


# Refactoring goals

## Split src/main.rs into per-concern modules

`src/main.rs` is growing long as commands accumulate. The clap struct definitions and
command handler functions for each subcommand should move into their own modules,
leaving `main.rs` as a thin dispatch layer (parse + call).

Natural split points:

| Module | Contents |
|---|---|
| `src/cmd/init.rs` | `InitArgs`, `cmd_init`, `install_git_hook`, `ensure_gitignore` |
| `src/cmd/check_token.rs` | `CheckTokenArgs`, `cmd_check_token` (or inline into `src/check_token.rs` directly) |
| `src/cmd/fetch.rs` | `FetchArgs`, `cmd_fetch` |
| `src/cmd/set_token.rs` | `SetTokenArgs`, `cmd_set_token` |
| `src/cmd/invoke.rs` | `RunArgs`, `cmd_run`, `cmd_dump_data`, `cmd_dump_defines` |
| `src/cmd/profile.rs` | `ProfileCmd`, `ProfileSub`, all `cmd_profile_*` functions |

`main.rs` retains only: `Cli`, `CommonArgs`, `Cmd` enum, `main()`, `load_config`, `resolve_profile`.


# Manual field testing

All invoke and download commands require a live Factorio install and/or mod portal
credentials and cannot be exercised in automated tests.

## Subgoal C / CC / BB: invoker + profiles + downloader

Run these after any change to `invoker.rs`, `profile.rs`, `downloader.rs`, `main.rs`,
or the helper mod files. Prerequisites: `FACTORIO_BIN` set or `factorio-bin` in config;
project initialised with `facts-and-oreos init`.

### init and profile lifecycle
- [ ] `facts-and-oreos init --factorio-bin <path>` in a clean directory: creates
      `.facts-and-oreos.toml`, `.profiles/default/mods/`, `.profiles/default/saves/`,
      helper mod files, `mod-settings.dat`, `modlist.json`; patches `.gitignore`
- [ ] `facts-and-oreos init` (no `--factorio-bin`): succeeds, prints note about missing binary
- [ ] `facts-and-oreos init` re-run without `--force`: exits with clear error
- [ ] `facts-and-oreos init --git` in a directory without `.git/`: runs `git init`,
      writes `.git/hooks/pre-commit`, hook is executable
- [ ] `facts-and-oreos init --git` with `core.hooksPath` set: hook written to configured dir
- [ ] `facts-and-oreos profile list`: shows `* default`
- [ ] `facts-and-oreos profile new myprofile`: creates `.profiles/myprofile/` with helper mod
- [ ] `facts-and-oreos profile set-default myprofile`: updates `.facts-and-oreos.toml`
- [ ] `facts-and-oreos profile clone default clone1`: clone1 has same modlist, no saves copied
- [ ] `facts-and-oreos profile delete default` while default is active: clear error
- [ ] `facts-and-oreos profile delete myprofile --force`: removes directory

### set-token and credentials
- [ ] `facts-and-oreos set-token <username> <token>`: writes
      `.profiles/default/player-data.json` at mode 0o600; file is valid JSON with
      `service-username` and `service-token` keys
- [ ] `facts-and-oreos --profile myprofile set-token <username> <token>`: writes to
      myprofile's `player-data.json`, not default's
- [ ] `facts-and-oreos fetch` without prior `set-token`: clear error naming the profile
      and pointing to https://factorio.com/profile

### fetch and zip cache
- [ ] `facts-and-oreos fetch <mod>`: resolves deps, downloads to `.profiles/default/mods/`,
      populates `.cache/mods/`, updates `modlist.json` and `mods.json`
- [ ] Re-run `fetch` of same mod: no HTTP request (cache hit), completes instantly
- [ ] `facts-and-oreos --profile myprofile fetch <mod>` after fetching to default:
      zip copied from `.cache/mods/` into myprofile without re-download
- [ ] Corrupt a zip in `.cache/mods/` (truncate it): next `fetch` re-downloads cleanly

### invoker
- [ ] `facts-and-oreos run`: loads `facts-and-oreos-helper-mod/empty` scenario, exits cleanly,
      no errors reported; temp dir is created under `/tmp/`
- [ ] `facts-and-oreos dump-data`: produces non-empty `data-raw-dump.json` in
      `.profiles/default/mods/`
- [ ] `facts-and-oreos dump-defines`: produces non-empty `defines.json` in
      `.profiles/default/mods/`; top-level key is `"defines"`
- [ ] Verify nothing is written to the user's real `~/.factorio/` during any invoke command
- [ ] Verify temp dir under `/tmp/` is left behind after a run (cleanup not yet implemented —
      track as a known issue; consider adding in a future pass)

### check-token and pre-commit hook
- [ ] `echo "abc$(python3 -c 'print(\"a\"*30)')def" | facts-and-oreos check-token`:
      exits 1, prints match with line number
- [ ] `echo "abc$(python3 -c 'print(\"a\"*29)')def" | facts-and-oreos check-token`:
      exits 0 (29 chars, not 30)
- [ ] `echo "abc$(python3 -c 'print(\"a\"*40)')def" | facts-and-oreos check-token`:
      exits 0 (40-char SHA1 run, not 30)
- [ ] `facts-and-oreos check-token --staged` with a clean index: exits 0
- [ ] Stage a file containing a 30-char hex string, then `git commit`: pre-commit hook
      fires, prints the file and line, aborts commit
- [ ] Stage a file with only a 40-char hex string: hook passes, commit proceeds