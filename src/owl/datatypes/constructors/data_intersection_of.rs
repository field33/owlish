use super::DatatypeDefinitionConstructor;
use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataIntersectionOf(
    #[serde(rename = "iri")] pub DataPropertyIRI,
    #[serde(rename = "datatype")] pub Box<DatatypeDefinitionConstructor>,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DataIntersectionOf {
    pub fn iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn datatype(&self) -> &DatatypeDefinitionConstructor {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
    iri: IRI, 
    datatype: DatatypeDefinitionConstructor,
    annotations: Array<Annotation>,
};
"#;
}
