use crate::owl::{Annotation, DataPropertyIRI, DatatypeIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyRange {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "datatypeIRI")]
    pub datatype_iri: DatatypeIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DataPropertyRange {
    pub fn new(
        data_property_iri: DataPropertyIRI,
        datatype_iri: DatatypeIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri,
            datatype_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataPropertyRange = {
    dataPropertyIRI: IRI,
    datatypeIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
