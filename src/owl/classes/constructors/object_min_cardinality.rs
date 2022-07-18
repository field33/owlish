use crate::owl::{ClassConstructor, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectMinCardinality(pub u64, pub ObjectPropertyIRI, pub Option<ClassIRI>);

impl From<ObjectMinCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectMinCardinality) -> Self {
        Box::new(ClassConstructor::ObjectMinCardinality(c))
    }
}
impl From<ObjectMinCardinality> for ClassConstructor {
    fn from(c: ObjectMinCardinality) -> Self {
        ClassConstructor::ObjectMinCardinality(c)
    }
}

impl ClassConstructor {
    pub fn object_min_cardinality(&self) -> Option<&ObjectMinCardinality> {
        match self {
            ClassConstructor::ObjectMinCardinality(d) => Some(d),
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
export type ObjectMinCardinality = [number, IRI, IRI | undefined];
"#;
}
