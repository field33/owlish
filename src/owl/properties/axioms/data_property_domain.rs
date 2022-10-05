use crate::owl::{Annotation, ClassIRI, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyDomain {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "classIRI")]
    pub class_iri: ClassIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DataPropertyDomain {
    pub fn new(
        data_property_iri: DataPropertyIRI,
        class_iri: ClassIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri,
            class_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataPropertyDomain = {
    dataPropertyIRI: IRI,
    classIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
