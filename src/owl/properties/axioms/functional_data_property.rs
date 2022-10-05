use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FunctionalDataProperty {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl FunctionalDataProperty {
    pub fn new(data_property_iri: DataPropertyIRI, annotations: Vec<Annotation>) -> Self {
        Self {
            data_property_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type FunctionalDataProperty = {
    dataPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
