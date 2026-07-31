pub struct HelperModFile {
    pub rel_path: &'static str,
    pub contents: &'static [u8],
}

pub fn all_files() -> &'static [HelperModFile] {
    &[
        HelperModFile {
            rel_path: "info.json",
            contents: include_bytes!("../facts-and-oreos-helper-mod/info.json"),
        },
        HelperModFile {
            rel_path: "scenarios/empty/control.lua",
            contents: include_bytes!("../facts-and-oreos-helper-mod/scenarios/empty/control.lua"),
        },
        HelperModFile {
            rel_path: "scenarios/empty/description.json",
            contents: include_bytes!(
                "../facts-and-oreos-helper-mod/scenarios/empty/description.json"
            ),
        },
        HelperModFile {
            rel_path: "scenarios/empty/mapgen-settings.json",
            contents: include_bytes!(
                "../facts-and-oreos-helper-mod/scenarios/empty/mapgen-settings.json"
            ),
        },
        HelperModFile {
            rel_path: "scenarios/dump-defines/control.lua",
            contents: include_bytes!(
                "../facts-and-oreos-helper-mod/scenarios/dump-defines/control.lua"
            ),
        },
        HelperModFile {
            rel_path: "scenarios/dump-defines/description.json",
            contents: include_bytes!(
                "../facts-and-oreos-helper-mod/scenarios/dump-defines/description.json"
            ),
        },
        HelperModFile {
            rel_path: "scenarios/dump-defines/mapgen-settings.json",
            contents: include_bytes!(
                "../facts-and-oreos-helper-mod/scenarios/dump-defines/mapgen-settings.json"
            ),
        },
    ]
}

/// Name of the helper mod directory inside a profile's mods dir.
pub const MOD_DIR_NAME: &str = "facts-and-oreos-helper-mod";
