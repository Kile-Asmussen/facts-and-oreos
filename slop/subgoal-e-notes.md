# Subgoal E: Factorio Global Lua Environment — Investigation Notes

## Scope clarification

Subgoal E is broader than just LuaHelpers. It covers the full Factorio global environment setup: all injected globals, modified stdlib functions, and stripped modules. require() (originally noted under Subgoal I) logically belongs here too but is deferred as complex.

## table_size()

- NOT in FLua source. Only a dead comment in lvm.c referencing it.
- Factorio injects it at engine level post-VM-init.
- Implementation: call luaH_size(t, 0) via FFI (non-fuzzy mode counts only non-nil slots).
- luaH_size walks the insertion-order linked list (firstadded chain) and the array part.
- O(n) in live entries, no cached count field. Fastest possible without a stored counter.
- For our purposes (mod dev tooling, tables < a few thousand entries), this is fine.

## FLua table model (insertion-order pairs/next)

- Each hash Node has next/prev pointers forming a doubly-linked list.
- Table has firstadded/lastadded head/tail pointers.
- luaH_newkey calls appendnode() to insert each new key at the tail.
- luaH_next: iterates array part first (indices 1..MAXASIZE=1024 in numeric order), then follows firstadded linked list for hash part.
- Deleted keys (set to nil) are moved to the END of the chain by checknilnode(), not removed. So luaH_size must still test ttisnil on each node.
- MAXASIZE = 1 << LUA_MAX_SEQUENTIAL_ARRAY_SIZE_BITS = 1024. Array-part keys 1..1024 always come before hash-part keys in iteration.

## math.random / math.randomseed

- Both are STOCK in FLua (call C stdlib rand()/srand()). No changes.
- Factorio replaces them at engine level after VM init.
- Plan: override from Rust after opening math library.
  - math.random: Rust closure backed by seeded PRNG in RefCell<SmallRng> (or similar). Pipeline is single-threaded, RefCell sufficient.
  - math.randomseed: no-op closure (Factorio docs: "has no effect").
  - Data stage seed: constant (Factorio seeds with a fixed value at data stage).
  - Control stage: would use map seed — out of scope for now.

## serpent

- Already bundled in core mod (vanilla .factorio/vanilla/core/). No action needed.

## log() / localised_print()

- Stub implementations:
  - If argument is a string: log() writes to log file, localised_print() writes to stdout.
  - If argument is a LocalisedString (table): use serpent.line() as a standin to render it readably. Full localisation evaluation deferred.
- Full localisation evaluation (Subgoal H): LocalisedString is an S-expression-like language over simple string substitution. Needed to detect missing localisation keys. Not complex, deferred.

## Stripped modules

- Remove from global env after opening stdlib: loadfile, dofile, coroutine, io, os.
- load(): restrict to text chunks only (mode argument ignored, binary chunks rejected).
- debug: keep only debug.getinfo() and debug.traceback().
- Trivial to implement: just set globals to nil or replace with restricted versions.

## require() — DEFERRED (see subgoal-i-notes.md)

- Complex design, belongs here conceptually but implementation deferred to when we tackle Subgoal I (mod loading pipeline).
- Path conventions (from libraries.txt):
  - mod-local: require("myfile") -> <modname>/myfile.lua
  - core lualib: require("util") -> core/lualib/util.lua  
  - cross-mod: require("__othermod__.file") -> othermod/file.lua
- Cannot use absolute paths outside mod root. ".." disabled.
- Cannot be used in console, event listeners, or remote.call().
- See subgoal-i-notes.md for dependency-gathering design.

## Implementation order for Subgoal E

1. table_size() via luaH_size FFI
2. math.random/randomseed override (RefCell PRNG)
3. Strip unavailable modules (nil out globals)
4. log() / localised_print() stubs
5. require() — separate subgoal item, blocked on mod loading design
