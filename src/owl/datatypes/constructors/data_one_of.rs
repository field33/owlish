use crate::owl::Literal;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataOneOf(pub Vec<Literal>);

impl From<DataOneOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c).into()
    }
}
impl From<DataOneOf> for DatatypeDefinitionConstructor {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataOneOf = Array<unknown>;
"#;
}
