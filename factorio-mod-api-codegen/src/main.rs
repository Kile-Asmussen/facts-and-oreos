/// Manual testing entry point — invoke with no args to download and regenerate,
/// or pass <prototype-api.json> <out-dir> to use a local file.
use std::path::PathBuf;

use factorio_mod_api_codegen::{codegen::rust, fetch, model::ApiDoc};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (json_path, out_dir) = match args.len() {
        1 => {
            let cache = PathBuf::from("target/api-cache");
            let json = fetch::prototype_api_json(&cache)
                .unwrap_or_else(|e| panic!("failed to fetch prototype-api.json: {e}"));
            (json, PathBuf::from("slop/codegen-out"))
        }
        3 => (PathBuf::from(&args[1]), PathBuf::from(&args[2])),
        _ => {
            eprintln!("Usage: factorio-mod-api-codegen [<prototype-api.json> <out-dir>]");
            std::process::exit(1);
        }
    };

    let json = std::fs::read_to_string(&json_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", json_path.display()));

    let doc: ApiDoc = serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", json_path.display()));

    eprintln!(
        "Loaded {} prototypes, {} types, {} defines (api_version {})",
        doc.prototypes.len(),
        doc.types.len(),
        doc.defines.len(),
        doc.api_version,
    );

    std::fs::create_dir_all(&out_dir)
        .unwrap_or_else(|e| panic!("failed to create {}: {e}", out_dir.display()));

    let rust_src = rust::generate(&doc);
    let out_path = out_dir.join("prototypes.rs");
    std::fs::write(&out_path, &rust_src)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));
    eprintln!("Wrote {} ({} bytes)", out_path.display(), rust_src.len());
}
