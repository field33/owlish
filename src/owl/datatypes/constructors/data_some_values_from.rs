use crate::owl::{Annotation, DataPropertyIRI, DatatypeRestriction};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataSomeValuesFrom {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "restriction")]
    pub restriction: DatatypeRestriction,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DataSomeValuesFrom {
    pub fn new(
        data_property_iri: DataPropertyIRI,
        restriction: DatatypeRestriction,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri,
            restriction,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataSomeValuesFrom = {
    dataPropertyIRI: IRI, 
    restriction: DatatypeRestriction,
    annotations: Array<Annotation>,
};
"#;
}
