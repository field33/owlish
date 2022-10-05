use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentDataProperties {
    #[serde(rename = "dataPropertyIRI1")]
    pub data_property_iri_1: DataPropertyIRI,
    #[serde(rename = "dataPropertyIRI2")]
    pub data_property_iri_2: DataPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl EquivalentDataProperties {
    pub fn new(
        data_property_iri_1: DataPropertyIRI,
        data_property_iri_2: DataPropertyIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri_1,
            data_property_iri_2,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type EquivalentDataProperties = {
    dataPropertyIRI1: IRI,
    dataPropertyIRI2: IRI,
    annotations: Array<Annotation>,
};
"#;
}
