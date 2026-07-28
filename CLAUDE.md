
# Factorio mod test suite

The 'claude fishing' suite of security hooks are installed, brief description in .claude/fishing.txt, read this before beginning as it contains crucial information about constrains for how to work within this project. Read the associated allow list files, and verify that the grep-glob mcp is available. 

Note: memories are not available. Use local files for tracking progress.

This Rust project has the purpose of recreating with fairly high accuracy the settings-stage and data-stage loading pipeline of factorio as well as a simpler simulation of the control-stage loading, available for integration in factorio mod development.

Four reference directories are provided:

./api/ contains a replica of the Factorio mods api documentation, with html files turned into txt to save on tokens when read.

./mlua/ contains the source code of the mlua Rust library, which provides lua integration in Rust:
https://docs.rs/mlua/latest/mlua/

./ftools/ which contains the source code of the factorio-rust-tools project:
https://docs.rs/factorio-exporter/0.9.0/factorio_exporter/
https://docs.rs/factorio-mod-api/0.3.0/factorio_mod_api/

./flua/ which contains the source code of the modified Lua 5.2 runtime that Factorio uses

./.factorio/vanilla/ which contains the five 'mods' provided for the vanilla experience of Factorio.

If any of these directories are missing, ask the user to run the appropriate make commands defined in the project Makefile.

Further directories of interest:

./slop/ is a sandbox directory with full, unrestricted file writing and reading access. Use this for tracking ongoing progress for future sessions, if needed, and for drafting design documents at user request. Use this directory for storing progress files.

Note: When writing files for claude to read, write them for claude, not necessarily for a human to read. Use Markdown formatting only if it aids claude's comprehension. Consider if tables are more readable to claude than lists, and so on.

The ./TODO.md file provides a very extensive plan for the progress.