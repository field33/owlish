use super::DatatypeDefinitionConstructor;
use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataIntersectionOf {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "datatype")]
    pub datatype: Box<DatatypeDefinitionConstructor>,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DataIntersectionOf {
    pub fn new(
        iri: DataPropertyIRI,
        datatype: Box<DatatypeDefinitionConstructor>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri: iri,
            datatype,
            annotations,
        }
    }
}

impl From<DataIntersectionOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataIntersectionOf) -> Self {
        DatatypeDefinitionConstructor::DataIntersectionOf(c).into()
    }
}

impl From<DataIntersectionOf> for DatatypeDefinitionConstructor {
    fn from(c: DataIntersectionOf) -> Self {
        DatatypeDefinitionConstructor::DataIntersectionOf(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataIntersectionOf = {
    dataPropertyIRI: IRI, 
    datatype: DatatypeDefinitionConstructor,
    annotations: Array<Annotation>,
};
"#;
}
