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

NOTE: `ApiToken` has no `Debug` impl to prevent accidental logging. Credentials stored at `$XDG_CONFIG_HOME/facts-and-oreos/credentials.json` — fishing hooks should deny-read this path.

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
- [x] C4. `facts-and-oreos` mod at `.factorio/mods/facts-and-oreos/`: `info.json`, `scenarios/empty/` (headless bootstrap), `scenarios/dump-defines/` (on_init defines dump)
- [x] C5. `run_headless(save: Option<&Path>, mods_dir, until_tick)`: `--load-scenario facts-and-oreos/empty` or `--load-game <path>`; returns structured `RunOutput` with parsed errors/warnings
- [x] C6. `dump_data(mods_dir)`: `--dump-data`; moves result to `.factorio/mods/data-raw-dump.json`
- [x] C7. `dump_defines(mods_dir)`: runs dump-defines scenario; moves result to `.factorio/mods/defines.json`
- [x] C8. CLI: `run [save]`, `dump-data`, `dump-defines`

NOTE: `find_binary_from_project` (reading `.factorio/project.toml`) is stubbed — only `FACTORIO_BIN` env var works currently.

NOTE: To speed up scenario loading, it may be beneficial to supply custom map generation settings (via `--map-gen-settings`) that produce the most minimal possible world — e.g. no resources, no enemies, smallest map size. This would reduce world generation time significantly for the empty and dump-defines scenarios.

NOTE: Consider adding a CLI command to launch Factorio for ordinary (non-headless) play against the project's mod set, to aid the mod developer in testing their work interactively without needing to manage Factorio's own mod directory.

NOTE: Investigate using Factorio's replay system as a basis for automated testing in mod development workflows — a replay encodes all player actions deterministically, which could allow replaying against modified mod code and checking for divergence or errors.

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

## Subgoal N: Claude Skill for Factorio modding

A skill giving descriptions for how to mod Factorio using this tool suite, as well as the Emmylua LSP as a plugin.

- TODO list for this subgoal goes here


# Manual field testing

## Subgoal C: headless invoker

The headless invoker outputs are complex enough that fully automated verification of correctness is impractical; instead, the following should be tested manually when modifying invoker or scenario code:

- `facts-and-oreos run`: loads the empty scenario, exits cleanly, no errors reported
- `facts-and-oreos dump-data`: produces a non-empty `data-raw-dump.json` in `.factorio/mods/`
- `facts-and-oreos dump-defines`: produces a non-empty `defines.json` in `.factorio/mods/`; verify it contains top-level keys like `"defines"`
- Verify that the isolated temp environment does not write anything into the user's real Factorio `write-data` directory
- Verify that after a run, the temp dir is cleaned up (currently it is not — consider adding cleanup)