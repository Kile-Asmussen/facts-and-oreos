
# A factorio data-stage mod loading simulation

Under construction using Spec-Driven development.

Once completed this project will include:

- A comprehensive simulation of the mod loading pipeline
  - Authentic replication of Factorio's Lua dialect
  - Access to in-game value definitions based on data dumps
  - Management of mod settings profiles
  - Comprehensive correctness checks of all created and edited prototypes
  - ID-cross checks of all loaded prototypes
  - Dynamic requirement downloading and packaging script generation based on `require` calls
  - Dynamic locale file stub generation based on prototypes
  - Lightweight Lua unit testing framework

- Mod profile handling
  - Automatic downloading of mod files, obeying all mod dependencies
  - Loading of vanilla 'mods' only as needed
  - Generation of data.raw JSON files for inspection via JQ-based tooling
  - Mod settings stored in JSON files rather than the game's custom data formats
  - Headless invocation of the game as 'source of truth' to catch any errors not found by the testing pipeline
  - Creation of replay-capable save files to serve as test cases for regression prevention in in-game scripting

- MCP tools for Factorio mod development
  - Optimized API reference lookups
  - Direct inspection of data.raw at any point in mod loading pipeline
  - Emmylua LSP with optional custom type annotations