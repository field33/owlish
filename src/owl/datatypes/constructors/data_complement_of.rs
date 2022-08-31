use crate::owl::{Annotation, DataPropertyIRI};

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataComplementOf(
    #[serde(rename = "iri")] pub DataPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DataComplementOf {
    pub fn iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.1
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
    iri: IRI,
    annotations: Array<Annotation>,
};
"#;
}
