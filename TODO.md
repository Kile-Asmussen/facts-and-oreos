# Goal: Replicate factorio mod loading pipeline

The purpose is to provide a fast sanity check system for factorio mod development.

Consider design options of each subgoal in dialogue with user and fill out a comprehensive to-do list of each subgoal.

## Subgoal A: Factorio mod api integration

Make or find a suitable factorio mod api integration.

Of interest, investigate whether factorio_mod_api is of use, available on https://docs.rs/factorio-mod-api/latest/factorio_mod_api/ also available for inspection in ./ftools/ directory.

Wiki page with details: https://wiki.factorio.com/Mod_portal_API

NOTE: Includes credential handling, which needs to be guarded against leaking through Claude accidentally reading the file using the fishing hooks.

CONSIDER: project-local mods caching vs using the factorio mods directory ~/.factorio/mods/

## Subgoal B: Implement Factorio mod downloader

Make a tool that takes a set of mods referenced by name and optionally version, download all mods and their prerequisites (available by inspecting their info.json files), unpack them to a suitable caching directory.

This should include a utility for updating a modlist.json file, same as the game.

## Subgoal C: A suite of tools to invoke factorio in headless mode

Make a tool for running factorio in headless mode with a select set of mods, for several purposes: dumping the data.raw JSON for that set of mods, checking generated logs/stdout printout to help diagnose errors, and extracting internally available values from the Factorio Lua engine (such as defines detailed ./api/defines.txt) by running empty-world scenarios for 1 tick with tiny mods installed that use an on_init event hook to dump data.

NOTE: this tool will need to edit the already installed list of mods, and so must take care to restore the mods that the user has already installed after each invocation. Alternatively investigate using docker to compartmentalize the headless factorio invocation.

## Subgoal D: Create mlua integration with Factorio Lua
 
Add Factorio Lua as an accessible implementation of lua in the mlua library.

Make sure that there is compliance with the descriptions of functionality in available as plain text in ./api/auxiliary/libraries.txt as well as the 'determinism' requirement posited by Factorio's design (though not quite as important for a simple checking tool).

NOTE: the require function in Factorio allows mods to load data out of core/lualib as well as files local to the mod, and from other mods using __modname__.luafile

## Subgoal E: Replicate LuaHelpers

LuaHelpers is a library available at the mod loading stage, and so needs to be made available to the code. Check ./api/classes/LuaHelpers.txt for information.

## Subgoal F: Code generation for implementing Factorio's prototypes

The entirety of Factorio's Prototype documentation is available in machine readable format, and code can therefore be generated for it. ./api/prototypes-api.json and ./api/runtime-api.json

Of particular interest: Serde-integrated Rust type definitions (in build.rs), and Lua annotations for the EmmyLua LSP (likely as an invoked tool).

## Subgoal G: Verification of integrity of prototypes

Using the generated prototype definitions, at the end of the simulated loading stages the protypes should be checked against the definitions. IDs should be cross referenced for consistency, numeric and string values should be checked for validity, paths to graphics and sounds should be checked for existence and file type, etc.

## Subgoal H: Localisation string verification

One problem in factorio mod development is localization strings. The mod loader simulation should be able to determing which localisation strings are not defined. 

NOTE: the behavior of the localised_name and localised_description fields of prototypes exclude them from ordinary linking to the default localisation string IDs, but also that they themselves contain additional localisation strings.

NOTE: there is a defaulting behavior for naming/description of item-building-recipe triplets:

https://wiki.factorio.com/Tutorial:Localisation#Default_Behavior(s)_for_finding_an_Unspecified_Localised_String

## Subgoal I: Mod loading

Once mod loading begins, several global Lua variables are defined in the factorio mod loading pipeline, which is available for inspection by each mod. The mods will have to be ordered in terms of their prerequisites, and loaded in a deterministic ordering scheme, as well as checking for cycling dependencies, etc.

NOTE: Determine how Factorio does the ordering of non-dependent mods, and replicate that if possible.

## Subgoal J: Setting stage simulation

The factorio startup loading process is split into a settings stage and a data stage. Between the two, the lua runtime is destroyed entirely. After the settings stage, the declared settings need to be available for the user to edit for the data stage run for instance through a suitable configuration file including the localised names/descriptions within the config file for improved usability.

## Subgoal H: Prototype stage simulation

This is the meat of the project, the data loading stage is where most of the checks implemented in subgoal G and H are applicable. It will however use much of the same logic as in I and J.

Furthermore this stage (and the settings stage) should implement some kind of logic for determining not only which prototypes are added with data:extend but also which prototypes are edited by each mod.

## Subgoal K: Control stage simulation

In factorio, when starting a scenario or loading a saved game, the control stage executes, letting the developer install hooks. (This again requires a fresh Lua VM instance.) While it is beyond the scope of this project to simulate invoking these hooks, there should be a sanity check for it, such as for localisation strings and various pitfalls (for instance each mod can only have one on_init hook installed.)

## Subgoal L: Design for a frictionless usage in mod development

The purpose of this tool is to be invoked within a development directory of a mod, and simulate the process of loading the mod into the game. Considerations need to be given to the user experience for mod developers.

This includes usage guides, sensible choice of caching directories, etc.