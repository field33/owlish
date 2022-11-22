use crate::owl::{Annotation, Axiom, ClassConstructor, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyDomain {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "cls")]
    pub cls: ClassConstructor,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DataPropertyDomain {
    pub fn new(
        data_property_iri: DataPropertyIRI,
        cls: ClassConstructor,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri,
            cls,
            annotations,
        }
    }
}

impl From<DataPropertyDomain> for Axiom {
    fn from(dpd: DataPropertyDomain) -> Self {
        Axiom::DataPropertyDomain(dpd)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataPropertyDomain = {
    dataPropertyIRI: IRI,
    cls: ClassConstructor,
    annotations: Array<Annotation>,
};
"#;
}
