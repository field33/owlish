use crate::owl::{Annotation, DataPropertyIRI};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubDataPropertyOf {
    #[serde(rename = "dataPropertyIRI")]
    pub subject_iri: DataPropertyIRI,
    #[serde(rename = "parentDataPropertyIRI")]
    pub parent_iri: DataPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl SubDataPropertyOf {
    pub fn new(
        subject_iri: DataPropertyIRI,
        parent_iri: DataPropertyIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            subject_iri,
            parent_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type SubDataProperty = {
    dataPropertyIRI: IRI,
    parentDataPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
