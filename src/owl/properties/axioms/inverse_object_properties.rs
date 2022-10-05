use crate::owl::{Annotation, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InverseObjectProperties {
    #[serde(rename = "objectPropertyIRI1")]
    pub object_property_iri_1: ObjectPropertyIRI,
    #[serde(rename = "objectPropertyIRI2")]
    pub object_property_iri_2: ObjectPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl InverseObjectProperties {
    pub fn new(
        object_property_iri_1: ObjectPropertyIRI,
        object_property_iri_2: ObjectPropertyIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            object_property_iri_1,
            object_property_iri_2,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type InverseObjectProperties = {
    objectPropertyIRI1: IRI,
    objectPropertyIRI2: IRI,
    annotations: Array<Annotation>,
};
"#;
}
