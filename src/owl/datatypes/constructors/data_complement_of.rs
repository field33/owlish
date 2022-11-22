use crate::owl::{Annotation, DataPropertyIRI};

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataComplementOf {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DataComplementOf {
    pub fn new(data_property_iri: DataPropertyIRI, annotations: Vec<Annotation>) -> Self {
        Self {
            data_property_iri,
            annotations,
        }
    }
}

impl From<DataComplementOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataComplementOf) -> Self {
        DatatypeDefinitionConstructor::DataComplementOf(c).into()
    }
}

impl From<DataComplementOf> for DatatypeDefinitionConstructor {
    fn from(c: DataComplementOf) -> Self {
        DatatypeDefinitionConstructor::DataComplementOf(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataComplementOf = {
    dataPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
