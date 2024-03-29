use crate::owl::{Annotation, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InverseFunctionalObjectProperty {
    #[serde(rename = "objectPropertyIRI")]
    pub object_property_iri: ObjectPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl InverseFunctionalObjectProperty {
    pub fn new(object_property_iri: ObjectPropertyIRI, annotations: Vec<Annotation>) -> Self {
        Self {
            object_property_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type InverseFunctionalObjectProperty = {
    objectPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
