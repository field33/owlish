use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::DataPropertyIRI;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataComplementOf(pub DataPropertyIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type DataComplementOf = IRI;
"#;

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
