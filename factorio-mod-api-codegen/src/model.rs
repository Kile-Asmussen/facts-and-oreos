use serde::Deserialize;

// ---------------------------------------------------------------------------
// Top level
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ApiDoc {
    pub application: String,
    pub application_version: String,
    pub api_version: u32,
    pub stage: String,
    pub prototypes: Vec<Prototype>,
    pub types: Vec<TypeConcept>,
    #[serde(default)]
    pub defines: Vec<Define>,
}

// ---------------------------------------------------------------------------
// BasicMember fields (shared, flattened into each struct)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct BasicMember {
    pub name: String,
    pub order: u32,
    pub description: String,
    #[serde(default)]
    pub lists: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
}

// ---------------------------------------------------------------------------
// Prototype
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Prototype {
    #[serde(flatten)]
    pub basic: BasicMember,
    #[serde(default)]
    pub visibility: Vec<String>,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub is_abstract: bool,
    pub typename: Option<String>,
    pub instance_limit: Option<u32>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub properties: Vec<Property>,
    pub custom_properties: Option<CustomProperties>,
}

// ---------------------------------------------------------------------------
// Type/Concept
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct TypeConcept {
    #[serde(flatten)]
    pub basic: BasicMember,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub is_abstract: bool,
    pub inline: bool,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(default)]
    pub properties: Vec<Property>,
}

// ---------------------------------------------------------------------------
// Define (recursive)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Define {
    #[serde(flatten)]
    pub basic: BasicMember,
    #[serde(default)]
    pub values: Vec<DefineValue>,
    #[serde(default)]
    pub subkeys: Vec<Define>,
}

#[derive(Debug, Deserialize)]
pub struct DefineValue {
    pub name: String,
    pub order: u32,
    pub description: String,
}

// ---------------------------------------------------------------------------
// Property
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(flatten)]
    pub basic: BasicMember,
    #[serde(default)]
    pub visibility: Vec<String>,
    pub alt_name: Option<String>,
    #[serde(rename = "override")]
    pub is_override: bool,
    #[serde(rename = "type")]
    pub ty: Type,
    pub optional: bool,
    pub default: Option<PropertyDefault>,
}

/// A property default is either a plain description string or a literal complex type object.
/// Reuses `Type` since that already handles both `Named(String)` and `Complex(Literal{..})`.
pub type PropertyDefault = Type;

// ---------------------------------------------------------------------------
// Type — the core recursive type expression
// ---------------------------------------------------------------------------

/// A type expression: either a plain named type (string) or a complex type.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Type {
    /// Plain named reference: "string", "uint32", "RecipePrototype", etc.
    Named(String),
    Complex(Box<ComplexType>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "complex_type", rename_all = "lowercase")]
pub enum ComplexType {
    Array {
        value: Type,
    },
    Dictionary {
        key: Type,
        value: Type,
    },
    Tuple {
        values: Vec<Type>,
    },
    Union {
        options: Vec<Type>,
        full_format: bool,
    },
    Literal {
        value: LiteralValue,
        description: Option<String>,
    },
    /// A type with a description attached — wraps another Type.
    #[serde(rename = "type")]
    TypeWrapper {
        value: Box<Type>,
        description: String,
    },
    /// Marker: properties live on the enclosing API member.
    Struct,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
}

// ---------------------------------------------------------------------------
// CustomProperties
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_prototype_api_json() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../reference/api/prototype-api.json"
        );
        let json = std::fs::read_to_string(path).expect("prototype-api.json not found");
        let doc: ApiDoc = serde_json::from_str(&json).expect("failed to parse prototype-api.json");
        assert_eq!(doc.stage, "prototype");
        assert!(doc.prototypes.len() > 100);
        assert!(doc.types.len() > 100);
        assert!(!doc.defines.is_empty());
    }
}

#[derive(Debug, Deserialize)]
pub struct CustomProperties {
    pub description: String,
    #[serde(default)]
    pub lists: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub images: Vec<Image>,
    pub key_type: Type,
    pub value_type: Type,
}
