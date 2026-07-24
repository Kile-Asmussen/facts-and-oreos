
# Factorio mod test suite

The 'claude fishing' suite of security hooks are installed, brief description in .claude/fishing.txt, read this before beginning as it contains crucial information about constrains for how to work within this project.

This Rust project has the purpose of recreating with fairly high accuracy the data-stage loading pipeline of factorio.

Three reference directories are provided:

./api/ contains a replica of the Factorio mods api documentation, with html files turned into txt to save on tokens when read.

./mlua/ contains the source code of the mlua Rust library, which provides lua integration in Rust.

./flua/ which contains the source code of the modified Lua 5.2 runtime that Factorio uses.

Further directories of interest:

./slop/ is a sandbox directory with full file