use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::DataPropertyIRI;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataUnionOf(pub DataPropertyIRI, pub Box<DatatypeDefinitionConstructor>);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type DataUnionOf = [IRI, DatatypeDefinitionConstructor];
"#;

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
