use crate::owl::{Annotation, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HasKey {
    #[serde(rename = "iri")]
    pub iri: ClassIRI,
    #[serde(rename = "objectProperties")]
    pub object_properties: Vec<ObjectPropertyIRI>,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl HasKey {
    pub fn new(
        iri: ClassIRI,
        object_properties: Vec<ObjectPropertyIRI>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            object_properties,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type HasKey = {
    iri: IRI, 
    objectProperties: Array<IRI>,
    annotations: Array<Annotation>,
};
"#;
}
