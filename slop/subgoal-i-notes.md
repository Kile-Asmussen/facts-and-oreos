# Subgoal I: Mod Loading Pipeline — Design Notes

## require() as a dependency-gathering mechanism

The custom require() interceptor is not just a loader — it is the primary means by which the pipeline builds up the packaging manifest and dependency graph for the mod being developed.

### Mod-local require: require("myfile") -> <modname>/myfile.lua

- Resolve path relative to mod root.
- Record the file in a packaging manifest (list of Lua files required by this mod).
- Manifest is used later to compile the mod zip, so only actually-required files are packaged.
- ".." path components: reject with error (matches Factorio behavior).

### Core lualib require: require("util") -> core/lualib/util.lua

- Resolve against .factorio/vanilla/core/lualib/.
- Do NOT add to packaging manifest (core lualib is provided by the game, not the mod).
- Do NOT add as a mod dependency.

### Cross-mod require: require("__othermod__.luafile") -> othermod/luafile.lua

- Parse __modname__ prefix to extract mod name.
- Add modname as a dependency to info.json (if not already present) with a >= current installed version constraint, or bare name if not installed yet.
- If modname is not currently installed in the active profile: immediately trigger download/install (same pipeline as `facts-and-oreos fetch`). This may entail a network request to the mod portal.
- After install, resolve and load the file from the newly installed mod's directory.
- Record in manifest that this mod depends on the external mod (for user visibility, not for packaging — the file lives in the other mod).

## LuaHelpers.is_valid_<path> — similar mechanism

- is_valid_sprite_path, is_valid_sound_path, etc.: check that a referenced asset file exists.
- If the path is local to the mod (e.g. "__mymod__/graphics/foo.png"): add file to packaging manifest.
- If the path references an external mod ("__othermod__/..."): add othermod as dependency, trigger install if missing (same as cross-mod require).
- Used in:
  - Prototype verification stage (Subgoal G): validate all prototype asset references.
  - Real-time during data stage loading: catch bad paths as they are registered.

## Packaging manifest

A file written out by the pipeline run, listing:
- All mod-local .lua files discovered via require().
- All mod-local asset files discovered via is_valid_* calls.
- All external mod dependencies discovered via __modname__ references.

Purpose: allow `facts-and-oreos pack` (or similar future command) to assemble a correct mod zip without manual file listing. Also surfaces unused files (files present in mod dir but not in manifest).

## Dependency graph / ordering

- Mods must be loaded in dependency order (prerequisites before dependents).
- Cycle detection required (error on circular deps).
- For non-dependent mods: determine Factorio's canonical ordering scheme (likely alphabetical by mod name, investigate against vanilla behavior or game source hints).
- Optional deps (?) and hidden optional deps ((?)) are loaded if present, skipped if absent — require() calls into them should still trigger install if the dep is declared optional and the mod is available.
- Incompatibility constraints (!): verified before loading begins.

## Global Lua environment per stage

Each stage (settings, data, control) gets a fresh Lua VM. The require() cache (package.loaded) is therefore also fresh per stage. Mods must re-require files in each stage they use them — this matches Factorio behavior.

## Notes on require() restrictions

- Cannot be called in: console, event listeners, remote.call().
- For our pipeline (data stage simulation) these restrictions are not exercised, but should be enforced if/when control stage simulation is added (Subgoal K).
