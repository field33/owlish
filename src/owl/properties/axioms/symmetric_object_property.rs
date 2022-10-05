use crate::owl::{Annotation, Axiom, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SymmetricObjectProperty {
    #[serde(rename = "objectPropertyIRI")]
    pub object_property_iri: ObjectPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl SymmetricObjectProperty {
    pub fn new(object_property_iri: ObjectPropertyIRI, annotations: Vec<Annotation>) -> Self {
        Self {
            object_property_iri,
            annotations,
        }
    }
}

impl From<SymmetricObjectProperty> for Axiom {
    fn from(sop: SymmetricObjectProperty) -> Self {
        Self::SymmetricObjectProperty(sop)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type SymmetricObjectProperty = {
    objectPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
