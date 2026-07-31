Subgoal D: mlua + FLua integration plan
========================================

## What FLua is

Lua 5.2.1 with the following Factorio-specific modifications:

| File | Modification |
|------|-------------|
| luaconf.h | `custom_pow` declaration + `luai_numpow` macro redirected to it |
| lmathlib.c | `custom_pow` implementation (non-static, deterministic exponentiation) |
| lapi.c | Added `lua_getlfield`, `lua_setlfield` (length-aware field get/set), `lua_isnumberorstringconvertabletonumber`, `lua_isstringornumberconvertabletostring`, `lua_pushunsigned`, `lua_tounsignedx` |
| lua.h | Declares all of the above new API functions |
| lapi.h | `extern "C"` on `index2addr` (C++ compilation assumption) |
| lua.hpp | Includes `<Lua/LuaCPPUtilities.hpp>` (C++ convenience wrappers) |
| LuaCPPUtilities.hpp | C++ overloads for `lua_pushstring`, `lua_getfield`, `lua_setfield`, templated `lua_pushnumber` |
| override_printf.h | Macro-redirects all printf/scanf family to `trio_*` -- NEVER included by any .c file in the repo; Factorio-internal only |

FLua compiles as C++ (not C): `extern "C"` on `index2addr` in lapi.h, C++ headers throughout.
`override_printf.h` appears to introduce a dependency on `trio` (a portable printf replacement), but this header is never `#include`d by any `.c` file in the repo -- it is dead code from Factorio's perspective and is NOT a build dependency for us.

## How mlua handles Lua compilation (the vendored path)

mlua-sys has two build paths:

1. **Non-vendored** (`find_normal.rs`): reads env vars `LUA_LIB`, `LUA_LIB_NAME`, `LUA_LINK`, or falls back to `pkg-config`.
2. **Vendored** (`find_vendored.rs`): delegates to `lua-src` crate (on crates.io), which ships upstream Lua source and compiles via `cc`. Exposes `Build::new().build(Lua52)` returning `Artifacts` with `print_cargo_metadata()`.

FLua is not on crates.io, so we cannot use the vendored path directly.

## Existing art: flua-src-rs

A project by fgardt (`./flua-src-rs/`) already implements exactly the `flua-src` crate pattern:
- `Build` / `Artifacts` API matching lua-src conventions
- `cc` build with `cpp(true)`, C++20, platform defines, excludes `lua.c`/`luac.c`
- `flua` git submodule pointing at `https://github.com/Rseding91/Factorio-Lua.git`
- No trio dependency -- confirms it compiles cleanly without it
- Version `1.1.0+factorio-2.0.46`, but targets an older FLua SHA

Since this crate is tiny (~200 lines), the decision is to **re-implement its functionality in-tree** rather than depend on it, incorporating our current FLua (from `./flua/`) and updating to our pinned SHA.

## Source management strategy: git submodules (DECIDED)

Rather than vendoring FLua source as plain files, use git submodules so updates are one command.

| Concern | Submodule | Vendored files |
|---------|-----------|----------------|
| Updating FLua | `git submodule update --remote`, commit new SHA | Manual copy |
| Pinned reproducibility | Yes, SHA committed | Yes, files frozen |
| First-time clone | Requires `--recurse-submodules` or `git submodule update --init` | Just works |
| CI | One extra step | Zero extra steps |
| Repo size | Lean | Source bloat |

**Mitigation for clone ergonomics:** build.rs emits a clear `panic!` with the exact remedy command if the submodule dir is empty. Makefile wraps `cargo build` with a prior `git submodule update --init --recursive`.

### Submodules to add

| Submodule | URL |
|-----------|-----|
| `flua` | `https://github.com/Rseding91/Factorio-Lua.git` |

## Crate architecture (decided)

```
workspace
├── flua-src/          new in-tree crate
│   ├── src/lib.rs     Build + Artifacts API (re-impl of flua-src-rs)
│   └── (no build.rs -- lib.rs IS the build helper, called from downstream)
├── flua-mlua-sys/     new in-tree crate
│   ├── build.rs       calls flua_src::Build::new().build(), emits link metadata
│   └── src/           mlua-sys lua52 FFI bindings + FLua extension symbols
└── facts-and-oreos    main crate, depends on flua-mlua-sys + upstream mlua
```

`flua` submodule lives at repo root as `./flua/`. The read-only reference copy is at `./reference/flua/` and must NOT be used by the production build -- `flua-src/src/lib.rs` must point `source_dir` at the submodule `./flua/src/`.

`flua-mlua-sys` links `cargo:links = "lua"` (same as mlua-sys) so upstream mlua can be told to use our lib via `LUA_LIB`/`LUA_LIB_NAME` env vars set in our build script -- avoiding the need to fork mlua itself.

## TODO list for Subgoal D

- [x] D1. Confirmed: override_printf.h never included by .c files; trio not a build dependency
- [x] D2. Create `flua-src/` crate:
  - [x] D2a. `src/lib.rs` with `Build` / `Artifacts` API (modelled on flua-src-rs)
  - [x] D2b. `cc::Build` config: `cpp(true)`, C++20, platform defines, excludes `lua.c`/`luac.c`
  - [x] D2c. `<Lua/LuaCPPUtilities.hpp>` not included by any `.c` file -- non-issue
  - [x] D2d. Panic with clear message if `flua/src/lua.h` missing (submodule guard)
  - [x] D2e. `flua` git submodule at `flua-src/flua/`; added via Makefile `submodules` target
- [x] D3. Create `flua-mlua-sys/` crate:
  - [x] D3a. mlua-sys `src/lua52/` FFI bindings as base
  - [x] D3b. FLua-only symbols: `lua_getlfield`, `lua_setlfield`, `lua_isnumberorstringconvertabletonumber`, `lua_isstringornumberconvertabletostring` (pushunsigned/tounsignedx already in standard lua52)
  - [x] D3c. `build.rs` calls `flua_src::Build::new().build()`, emits link metadata
  - [x] D3d. `cargo:links = "lua"` set
- [x] D4. Validate upstream mlua integration → shim approach works with mlua 0.12 + mlua-sys 0.11.

  `[patch.crates-io]` shim (`mlua-sys-shim`) re-exports `flua-mlua-sys`, satisfying `links = "lua"` uniqueness. mlua 0.10 failed (targeted mlua-sys 0.6.x API); mlua 0.12.0 targets mlua-sys 0.11.0, which our `flua-mlua-sys` matches exactly. Three constants missing from our lib.rs (`LUA_MAX_UPVALUES`, `LUA_TRACEBACK_STACK`, `SYS_MIN_ALIGN`) were added; `cargo test` passes clean. No fork of mlua needed.
- [x] D5. Smoke-test (all pass via `cargo test`):
  - [x] D5a. Open FLua state, execute `1 + 1`, assert result == 2.0
  - [x] D5b. Call `lua_getlfield` via FFI, verify `math` global is a table
  - [x] D5c. `tostring(2^10)` == `"1024"` (%.14g, no trailing .0 -- standard Lua 5.2 behaviour, not divergent)
- [x] D6. Moved to Subgoal I: custom `require` loader implemented via mlua high-level API alongside the rest of stage environment setup (data, mods, settings globals), not as raw FFI in flua-mlua-sys.
## Resolved questions

- **trio**: `override_printf.h` appears to introduce a trio dependency but is never included by any `.c` file -- dead code, not a build concern
- **C++ ABI**: FLua's `LUA_API = extern "C"` means Rust FFI bindings are identical to standard lua52; extensions are purely additive
- **`LuaCPPUtilities.hpp`**: only needed when Factorio includes lua.hpp; our build only exposes the C API headers, so this is a non-issue for the library build
- **mlua version**: Use mlua 0.12.0 (mlua-sys 0.11.0 API). mlua 0.10 is incompatible (mlua-sys 0.6.x API). Shim version must match mlua's mlua-sys requirement exactly.
