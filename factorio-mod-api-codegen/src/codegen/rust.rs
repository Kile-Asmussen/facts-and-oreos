use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::model::{ApiDoc, ComplexType, LiteralValue, Prototype, Property, Type, TypeConcept};

/// Generate a complete Rust source file from the API doc.
pub fn generate(doc: &ApiDoc) -> String {
    let mut tokens = TokenStream::new();

    tokens.extend(preamble());

    for t in &doc.types {
        tokens.extend(emit_type_concept(t, doc));
    }

    for p in &doc.prototypes {
        tokens.extend(emit_prototype(p, doc));
    }

    let file: syn::File = syn::parse2(tokens).expect("generated invalid Rust tokens");
    prettyplease::unparse(&file)
}

fn preamble() -> TokenStream {
    quote! {
        use serde::{Deserialize, Serialize};
        use factorio_mod_api_codegen::Visitable;
    }
}

// ---------------------------------------------------------------------------
// Type concepts
// ---------------------------------------------------------------------------

fn emit_type_concept(t: &TypeConcept, doc: &ApiDoc) -> TokenStream {
    let name = ident(&t.basic.name);

    match &t.ty {
        // "builtin" — skip, mapped at use sites
        Type::Named(s) if s == "builtin" => TokenStream::new(),

        // Named string alias → newtype struct for visitor enumeration
        Type::Named(target) if target == "string" => {
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
                pub struct #name(pub String);
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }
                impl AsRef<str> for #name {
                    fn as_ref(&self) -> &str { &self.0 }
                }
            }
        }

        // Named non-string alias → transparent type alias
        Type::Named(target) => {
            let target = map_builtin_name(target);
            quote! {
                pub type #name = #target;
            }
        }

        Type::Complex(ct) => match ct.as_ref() {
            // All-literal union → string enum
            ComplexType::Union { options, .. } if all_literal_strings(options) => {
                emit_string_enum(&name, options)
            }

            // General union → internally tagged or untagged enum
            ComplexType::Union { options, full_format } => {
                emit_union_enum(&name, options, *full_format, doc)
            }

            // Struct type → struct with optional parent flatten
            ComplexType::Struct => {
                emit_struct(&name, t.parent.as_deref(), &t.properties, doc)
            }

            // Array alias
            ComplexType::Array { value } => {
                let inner = type_to_tokens(value, doc);
                quote! { pub type #name = Vec<#inner>; }
            }

            // Dictionary alias
            ComplexType::Dictionary { key, value } => {
                let k = type_to_tokens(key, doc);
                let v = type_to_tokens(value, doc);
                quote! { pub type #name = std::collections::HashMap<#k, #v>; }
            }

            // Tuple alias
            ComplexType::Tuple { values } => {
                let elems: Vec<_> = values.iter().map(|v| type_to_tokens(v, doc)).collect();
                quote! { pub type #name = (#(#elems),*); }
            }

            // TypeWrapper — alias the inner type
            ComplexType::TypeWrapper { value, .. } => {
                let inner = type_to_tokens(value, doc);
                quote! { pub type #name = #inner; }
            }

            ComplexType::Literal { .. } => TokenStream::new(),
        },
    }
}

// ---------------------------------------------------------------------------
// Prototypes
// ---------------------------------------------------------------------------

fn emit_prototype(p: &Prototype, doc: &ApiDoc) -> TokenStream {
    let name = ident(&p.basic.name);
    emit_struct(&name, p.parent.as_deref(), &p.properties, doc)
}

// ---------------------------------------------------------------------------
// Struct emission
// ---------------------------------------------------------------------------

fn emit_struct(name: &Ident, parent: Option<&str>, props: &[Property], doc: &ApiDoc) -> TokenStream {
    let parent_field = parent.map(|p| {
        let parent_ident = ident(p);
        quote! {
            #[serde(flatten)]
            pub parent: #parent_ident,
        }
    });

    let fields: Vec<TokenStream> = props.iter().map(|p| emit_field(p, doc)).collect();

    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
        pub struct #name {
            #parent_field
            #(#fields)*
        }
    }
}

fn emit_field(prop: &Property, doc: &ApiDoc) -> TokenStream {
    let raw_name = &prop.basic.name;
    let field_name = sanitize_field_name(raw_name);
    let field_ident = ident(&field_name);
    let ty = type_to_tokens(&prop.ty, doc);

    let rename = if field_name != *raw_name {
        quote! { #[serde(rename = #raw_name)] }
    } else {
        TokenStream::new()
    };

    if prop.optional {
        quote! {
            #rename
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #field_ident: Option<#ty>,
        }
    } else {
        quote! {
            #rename
            pub #field_ident: #ty,
        }
    }
}

// ---------------------------------------------------------------------------
// Enum emission
// ---------------------------------------------------------------------------

fn emit_string_enum(name: &Ident, options: &[Type]) -> TokenStream {
    let variants: Vec<TokenStream> = options
        .iter()
        .filter_map(|opt| {
            if let Type::Complex(ct) = opt {
                if let ComplexType::Literal { value: LiteralValue::String(s), .. } = ct.as_ref() {
                    let variant_name = ident(&literal_string_to_ident(s));
                    return Some(quote! {
                        #[serde(rename = #s)]
                        #variant_name,
                    });
                }
            }
            None
        })
        .collect();

    let display_arms: Vec<TokenStream> = options
        .iter()
        .filter_map(|opt| {
            if let Type::Complex(ct) = opt {
                if let ComplexType::Literal { value: LiteralValue::String(s), .. } = ct.as_ref() {
                    let variant_name = ident(&literal_string_to_ident(s));
                    return Some(quote! { #name::#variant_name => #s, });
                }
            }
            None
        })
        .collect();

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
        #[serde(rename_all = "kebab-case")]
        pub enum #name {
            #(#variants)*
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self { #(#display_arms)* })
            }
        }
    }
}

fn emit_union_enum(name: &Ident, options: &[Type], full_format: bool, doc: &ApiDoc) -> TokenStream {
    // Try internally-tagged: all options must be TypeWrapper→Named struct with a literal `type` field
    if full_format {
        if let Some(tagged) = try_emit_tagged_enum(name, options, doc) {
            return tagged;
        }
    }

    let variants: Vec<TokenStream> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let (variant_name, ty) = union_variant(opt, i, doc);
            match ty {
                Some(t) => quote! { #variant_name(#t), },
                None => quote! { #variant_name, },
            }
        })
        .collect();

    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
        #[serde(untagged)]
        pub enum #name {
            #(#variants)*
        }
    }
}

/// Attempt to emit a `#[serde(tag = "type")]` enum.
/// Returns None if any option lacks a resolvable tag value.
fn try_emit_tagged_enum(name: &Ident, options: &[Type], doc: &ApiDoc) -> Option<TokenStream> {
    let variants: Vec<TokenStream> = options
        .iter()
        .map(|opt| {
            let (struct_name, description) = unwrap_type_wrapper(opt)?;
            let tag_value = resolve_type_tag(struct_name, description, doc)?;
            let variant_ident = ident(&to_pascal_case(struct_name));
            let struct_ident = ident(struct_name);
            Some(quote! {
                #[serde(rename = #tag_value)]
                #variant_ident(#struct_ident),
            })
        })
        .collect::<Option<Vec<_>>>()?;

    Some(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
        #[serde(tag = "type")]
        pub enum #name {
            #(#variants)*
        }
    })
}

/// If `ty` is a TypeWrapper, return (struct_name, description).
fn unwrap_type_wrapper<'a>(ty: &'a Type) -> Option<(&'a str, &'a str)> {
    if let Type::Complex(ct) = ty {
        if let ComplexType::TypeWrapper { value, description } = ct.as_ref() {
            if let Type::Named(s) = value.as_ref() {
                return Some((s.as_str(), description.as_str()));
            }
        }
    }
    None
}

/// Find the literal string value of the `type` property on a named struct in doc.types,
/// or extract it from the TypeWrapper description (format: `` `'tag-value'` ``).
fn resolve_type_tag<'a>(struct_name: &str, description: &'a str, doc: &ApiDoc) -> Option<String> {
    // First: look for a literal `type` property on the struct in doc.types
    if let Some(tc) = doc.types.iter().find(|t| t.basic.name == struct_name) {
        if let Some(type_prop) = tc.properties.iter().find(|p| p.basic.name == "type") {
            if let Type::Complex(ct) = &type_prop.ty {
                if let ComplexType::Literal { value: LiteralValue::String(s), .. } = ct.as_ref() {
                    return Some(s.clone());
                }
            }
        }
    }

    // Fallback: parse description like "`'accumulator'`" or "Loaded when the `type` is `\"throw\"`."
    extract_tag_from_description(description)
}

fn extract_tag_from_description(desc: &str) -> Option<String> {
    // Match `'tag'` style (AnyPrototype options)
    if let Some(start) = desc.find("`'") {
        let rest = &desc[start + 2..];
        if let Some(end) = rest.find("'`") {
            return Some(rest[..end].to_string());
        }
    }
    // Match `"tag"` style (CapsuleAction-style descriptions)
    if let Some(start) = desc.find("`\"") {
        let rest = &desc[start + 2..];
        if let Some(end) = rest.find("\"`") {
            return Some(rest[..end].to_string());
        }
    }
    None
}

fn union_variant(opt: &Type, idx: usize, doc: &ApiDoc) -> (Ident, Option<TokenStream>) {
    match opt {
        Type::Named(s) => {
            let ty = type_to_tokens(opt, doc);
            (ident(&to_pascal_case(s)), Some(quote! { Box<#ty> }))
        }
        Type::Complex(ct) => match ct.as_ref() {
            ComplexType::Literal { value, .. } => {
                let name = match value {
                    LiteralValue::String(s) => ident(&literal_string_to_ident(s)),
                    LiteralValue::Number(n) => ident(&format!("Literal{}", n.abs() as u64)),
                    LiteralValue::Bool(b) => ident(&format!("Literal{}", if *b { "True" } else { "False" })),
                };
                (name, None)
            }
            ComplexType::Struct => (ident(&format!("Struct{idx}")), None),
            ComplexType::Array { value } => {
                let inner = type_to_tokens(value, doc);
                (ident(&format!("Array{idx}")), Some(quote! { Vec<#inner> }))
            }
            ComplexType::TypeWrapper { value, .. } => {
                let ty = type_to_tokens(value, doc);
                (ident(&format!("Wrapped{idx}")), Some(ty))
            }
            _ => {
                let ty = type_to_tokens(opt, doc);
                (ident(&format!("Variant{idx}")), Some(ty))
            }
        },
    }
}

// ---------------------------------------------------------------------------
// Type → TokenStream mapping
// ---------------------------------------------------------------------------

fn type_to_tokens(ty: &Type, doc: &ApiDoc) -> TokenStream {
    match ty {
        Type::Named(s) => map_builtin_name(s),
        Type::Complex(ct) => match ct.as_ref() {
            ComplexType::Array { value } => {
                let inner = type_to_tokens(value, doc);
                quote! { Vec<#inner> }
            }
            ComplexType::Dictionary { key, value } => {
                let k = type_to_tokens(key, doc);
                let v = type_to_tokens(value, doc);
                quote! { std::collections::HashMap<#k, #v> }
            }
            ComplexType::Tuple { values } => {
                let elems: Vec<_> = values.iter().map(|v| type_to_tokens(v, doc)).collect();
                quote! { (#(#elems),*) }
            }
            ComplexType::Union { .. } => quote! { serde_json::Value },
            ComplexType::Literal { value, .. } => match value {
                LiteralValue::String(_) => quote! { String },
                LiteralValue::Number(_) => quote! { f64 },
                LiteralValue::Bool(_) => quote! { bool },
            },
            ComplexType::TypeWrapper { value, .. } => type_to_tokens(value, doc),
            ComplexType::Struct => quote! { serde_json::Value },
        },
    }
}

fn map_builtin_name(s: &str) -> TokenStream {
    if s.contains('.') {
        return quote! { u32 };
    }
    match s {
        "string" | "LocalisedString" => quote! { String },
        "boolean" | "bool" => quote! { bool },
        "number" | "double" | "float" | "MaterialAmountType" | "FluidAmount" | "Weight"
        | "RealOrientation" | "EffectValue" => quote! { f64 },
        "uint8" => quote! { u8 },
        "uint16" | "ItemStackIndex" | "LogisticFilterIndex" | "SpriteSizeType" => quote! { u16 },
        "uint32" | "ItemCountType" | "FluidBoxLinkedConnectionID" => quote! { u32 },
        "uint64" | "MapTick" => quote! { u64 },
        "int8" => quote! { i8 },
        "int16" => quote! { i16 },
        "int32" => quote! { i32 },
        "int64" => quote! { i64 },
        "table" | "DataExtendMethod" => quote! { serde_json::Value },
        other => {
            let id = ident(other);
            quote! { #id }
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn all_literal_strings(options: &[Type]) -> bool {
    options.iter().all(|opt| {
        matches!(
            opt,
            Type::Complex(ct) if matches!(ct.as_ref(),
                ComplexType::Literal { value: LiteralValue::String(_), .. }
            )
        )
    })
}

fn ident(s: &str) -> Ident {
    const KEYWORDS: &[&str] = &[
        "type", "override", "abstract", "use", "move", "loop", "match",
        "mod", "ref", "self", "struct", "enum", "impl", "trait", "where",
        "fn", "let", "mut", "pub", "in", "if", "else", "for", "while",
        "return", "true", "false", "as", "box", "break", "const", "continue",
        "crate", "dyn", "extern", "static", "super", "unsafe", "yield",
    ];
    if KEYWORDS.contains(&s) {
        Ident::new_raw(s, Span::call_site())
    } else {
        Ident::new(s, Span::call_site())
    }
}

fn sanitize_field_name(s: &str) -> String {
    s.replace('-', "_")
}

/// Convert kebab-case or snake_case to PascalCase.
fn to_pascal_case(s: &str) -> String {
    s.split(|c| c == '-' || c == '_' || c == ' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert an arbitrary string literal value to a valid PascalCase Rust identifier.
fn literal_string_to_ident(s: &str) -> String {
    if s.is_empty() {
        return "LiteralEmpty".to_string();
    }
    let mapped = match s {
        "=" => "Eq",
        "!=" => "Ne",
        "<" => "Lt",
        "<=" => "Le",
        ">" => "Gt",
        ">=" => "Ge",
        "≠" => "NotEqual",
        "≤" => "LessEqual",
        "≥" => "GreaterEqual",
        other => {
            let cleaned: String = other.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect();
            return format!("Literal{}", to_pascal_case(&cleaned));
        }
    };
    format!("Literal{mapped}")
}
