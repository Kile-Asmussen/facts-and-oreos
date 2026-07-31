use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("prototypes.rs");

    let force = std::env::var("REGENERATE_PROTOTYPES").is_ok();
    if dest.exists() && !force {
        println!("cargo:rerun-if-env-changed=REGENERATE_PROTOTYPES");
        return;
    }

    let cache = PathBuf::from("target/api-cache");
    let json_path = factorio_mod_api_codegen::fetch::prototype_api_json(&cache)
        .expect("failed to fetch prototype-api.json");

    let json = std::fs::read_to_string(&json_path)
        .expect("failed to read prototype-api.json");
    let doc: factorio_mod_api_codegen::model::ApiDoc =
        serde_json::from_str(&json).expect("failed to parse prototype-api.json");

    let rust_src = factorio_mod_api_codegen::codegen::rust::generate(&doc);
    std::fs::write(&dest, rust_src).expect("failed to write prototypes.rs");

    println!("cargo:rerun-if-env-changed=REGENERATE_PROTOTYPES");
    println!("cargo:rerun-if-changed=target/api-cache/prototype-api.json");
}
