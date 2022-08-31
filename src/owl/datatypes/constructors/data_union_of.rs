use super::DatatypeDefinitionConstructor;
use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataUnionOf(
    #[serde(rename = "iri")] pub DataPropertyIRI,
    #[serde(rename = "datatype")] pub Box<DatatypeDefinitionConstructor>,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl From<DataUnionOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataUnionOf) -> Self {
        DatatypeDefinitionConstructor::DataUnionOf(c).into()
    }
}
impl From<DataUnionOf> for DatatypeDefinitionConstructor {
    fn from(c: DataUnionOf) -> Self {
        DatatypeDefinitionConstructor::DataUnionOf(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataUnionOf = {
    iri: IRI, 
    datatype: DatatypeDefinitionConstructor,
    annotations: Array<Annotation>,
};
"#;
}
