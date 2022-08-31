use crate::owl::{Annotation, DataPropertyIRI, DatatypeIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyRange(
    #[serde(rename = "dataPropertyIRI")] pub DataPropertyIRI,
    #[serde(rename = "datatypeIRI")] pub DatatypeIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DataPropertyRange {
    pub fn iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn datatype_iri(&self) -> &DatatypeIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
