use crate::owl::{Annotation, Axiom, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SymmetricObjectProperty(
    #[serde(rename = "objectPropertyIRI")] pub ObjectPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl SymmetricObjectProperty {
    pub fn object_property_iri(&self) -> &ObjectPropertyIRI {
        &self.0
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.1
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
