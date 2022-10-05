use super::DatatypeDefinitionConstructor;
use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataIntersectionOf {
    pub iri: DataPropertyIRI,
    pub datatype: Box<DatatypeDefinitionConstructor>,
    pub annotations: Vec<Annotation>,
}

impl DataIntersectionOf {
    pub fn new(
        iri: DataPropertyIRI,
        datatype: Box<DatatypeDefinitionConstructor>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
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
    iri: IRI, 
    datatype: DatatypeDefinitionConstructor,
    annotations: Array<Annotation>,
};
"#;
}
