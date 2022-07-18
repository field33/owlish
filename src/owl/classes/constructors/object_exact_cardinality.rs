use crate::owl::{ClassConstructor, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectExactCardinality(pub u64, pub ObjectPropertyIRI, pub Option<ClassIRI>);

impl From<ObjectExactCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectExactCardinality) -> Self {
        Box::new(ClassConstructor::ObjectExactCardinality(c))
    }
}
impl From<ObjectExactCardinality> for ClassConstructor {
    fn from(c: ObjectExactCardinality) -> Self {
        ClassConstructor::ObjectExactCardinality(c)
    }
}

impl ClassConstructor {
    pub fn object_exact_cardinality(&self) -> Option<&ObjectExactCardinality> {
        match self {
            ClassConstructor::ObjectExactCardinality(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * [cardinality, ObjectProperty IRI, optinal Class IRI]
 */
export type ObjectExactCardinality = [number, IRI, IRI | undefined];
"#;
}
