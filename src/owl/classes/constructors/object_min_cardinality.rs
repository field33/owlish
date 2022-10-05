use crate::owl::{ClassConstructor, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectMinCardinality {
    pub value: u64,
    #[serde(rename = "objectPropertyIRI")]
    pub object_property_iri: ObjectPropertyIRI,
    pub cls: Option<ClassIRI>,
}

impl ObjectMinCardinality {
    pub fn new(value: u64, object_property_iri: ObjectPropertyIRI, cls: Option<ClassIRI>) -> Self {
        Self {
            value,
            object_property_iri,
            cls,
        }
    }
}

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
 */
export type ObjectMinCardinality = {
    value: number, 
    objectPropertyIRI: IRI, 
    cls: IRI | undefined
};
"#;
}
