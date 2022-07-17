use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::DataPropertyIRI;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataIntersectionOf(pub DataPropertyIRI, pub Box<DatatypeDefinitionConstructor>);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type DataIntersectionOf = [IRI, DatatypeDefinitionConstructor];
"#;

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
