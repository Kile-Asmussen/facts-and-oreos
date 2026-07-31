# Subgoal F: Prototype API Codegen — Design Notes

## Crate structure

Three crates:
- `factorio-mod-api-codegen` — library + manual-testing binary. Contains model, codegen, fetch, visitor.
- `factorio-mod-api-codegen-derive` — proc-macro crate, companion to above, exports `#[derive(Visitable)]`.
- Main crate uses `factorio-mod-api-codegen` as both a build-dep (for build.rs) and a runtime dep (for Visitable trait impls used in generated code).

## Source of truth

`https://lua-api.factorio.com/<version>/prototype-api.json` — downloaded by `fetch::prototype_api_json()` to `target/api-cache/`. Gitignored. Version is hardcoded in `fetch.rs` (currently 2.1.12). No zip archive — individual JSON files fetched directly.

`build.rs` skips regeneration if `OUT_DIR/prototypes.rs` exists, reruns on `REGENERATE_PROTOTYPES=1` env var.

## Model (model.rs)

Key design decisions:
- `Type` is an untagged serde enum: `Named(String)` for plain string refs, `Complex(Box<ComplexType>)` for objects.
- `ComplexType` uses `#[serde(tag = "complex_type")]` with variants: array, dictionary, tuple, union, literal, type (TypeWrapper), struct.
- `PropertyDefault` is `pub type PropertyDefault = Type` — not a custom enum. A custom untagged enum failed to parse `{"complex_type":"literal","value":0}` form.
- `LiteralValue` is untagged: `String(String) | Number(f64) | Bool(bool)`.

## Codegen (codegen/rust.rs)

### Type mapping
- Named string types (42 total, all `*ID` + Energy/FileName/etc.) → newtype structs `struct FooID(pub String)` with `Display`, `AsRef<str>`, and `Visitable` (leaf-only).
- Named non-string types → `pub type Foo = PrimitiveType`.
- Builtins mapped: string→String, boolean→bool, number/double/float→f64, uint*/int* → Rust integer types, table/DataExtendMethod→serde_json::Value, dotted names (defines.x)→u32.
- Struct complex types → `#[derive(..., Visitable)] pub struct Foo { #[serde(flatten)] pub parent: Bar, ... }`.
- All-literal-string unions → `pub enum Foo` with `#[serde(rename = "...")]` variants + `Display` impl.
- Mixed unions: if `full_format=true` AND all options are TypeWrapper→Named with a resolvable tag → `#[serde(tag = "type")]` enum. Tag resolved by: (1) literal `type` property on struct in doc.types, (2) description string parsing (`` `'tag'` `` or `` `"tag"` `` patterns). Falls back to `#[serde(untagged)]` if unresolvable.
- AnyPrototype (260 variants) uses description-based tag extraction since prototype `type` fields are `string` not literal.
- Array/dict/tuple → type aliases.
- TypeWrapper → alias to inner type.

### Union variant naming
- Untagged union with TypeWrapper options: variant name = PascalCase of struct name, type = `Box<StructName>`.
- Untagged union with literal options: variant name from `literal_string_to_ident()` (operator symbols mapped: =→Eq, !=→Ne, etc.; others cleaned+PascalCased with Literal prefix).
- Array variants → `Array{idx}(Vec<Inner>)`, struct variants → `Struct{idx}`, others → `Variant{idx}`.

### Field handling
- Hyphens in field names → underscores, `#[serde(rename = "original-name")]` added.
- Rust keywords → `r#keyword` via `Ident::new_raw`.
- Optional fields → `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`.

## Visitor pattern (visitor.rs + derive)

`Visitable: Any` trait with `visit_leaf(&self, visitor) -> bool` and `visit_node(&self, visitor)`. `Visitor` trait with `grab(&dyn Any) -> bool` and `visit(&dyn Visitable, entry: String)`. `Collector<T>` implements `Visitor`, collects `(Path, T)` pairs.

`enter`/`exit` eliminated — path segment passed directly to `visit()`, pushed/popped inside `Collector::visit`.

Blanket impls in visitor.rs:
- Primitives (u8..i64, bool, f32, f64, String) — leaf only via macro.
- `serde_json::Value` — no-op (neither leaf nor node).
- `Box<T>` — delegates to inner.
- `Vec<T>`, `[T]` — node, iterates with index as path segment.
- `HashMap<K: ToString, V>`, `BTreeMap<K: ToString, V>` — node, key.to_string() as path segment. `ToString` (not `AsRef<str>`) so newtype IDs and enums work as keys.
- `Option<T>` — delegates to inner or no-op on None.
- Tuples `(A,B)`, `(A,B,C)`, `(A,B,C,D)` — node, numeric path segments.

`#[derive(Visitable)]` (proc macro in -derive crate):
- Named-field struct → `visit_leaf` calls `grab(self)`, `visit_node` calls `visitor.visit(&self.field, "field")` for each field. `r#` prefix stripped from field name string.
- Newtype struct (single unnamed field) → `visit_leaf` only.
- Enum with single-field tuple variant → `visitor.visit(v, "VariantName")` (no `.as_ref()` — Box impl handles boxing transparently).
- Unit/multi-field variants → no-op in visit_node.

## Output wiring

Generated file written to `OUT_DIR/prototypes.rs`. Included in main crate via `src/prototypes.rs`:
```rust
#[allow(non_snake_case, dead_code, clippy::large_enum_variant)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/prototypes.rs"));
}
pub use generated::*;
```
`#![allow]` (inner attribute) is invalid in `include!` context outside a module, so the allow is on the wrapping `mod generated {}` instead.

Generated file imports: `use serde::{Deserialize, Serialize}; use factorio_mod_api_codegen::Visitable;` — so `factorio-mod-api-codegen` must be a runtime dependency of the main crate.

## Deferred

- F4: defines codegen (recursive subkey enums) — not needed until prototype verification uses define values.
- F5: EmmyLua annotation generation — not needed until LSP support subgoal.
