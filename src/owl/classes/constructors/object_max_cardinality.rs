use crate::owl::{ClassConstructor, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectMaxCardinality(pub u64, pub ObjectPropertyIRI, pub Option<ClassIRI>);

impl From<ObjectMaxCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectMaxCardinality) -> Self {
        Box::new(ClassConstructor::ObjectMaxCardinality(c))
    }
}
impl From<ObjectMaxCardinality> for ClassConstructor {
    fn from(c: ObjectMaxCardinality) -> Self {
        ClassConstructor::ObjectMaxCardinality(c)
    }
}

impl ClassConstructor {
    pub fn object_max_cardinality(&self) -> Option<&ObjectMaxCardinality> {
        match self {
            ClassConstructor::ObjectMaxCardinality(d) => Some(d),
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
export type ObjectMaxCardinality = [number, IRI, IRI | undefined];
"#;
}
