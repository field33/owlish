use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentDataProperties(
    #[serde(rename = "dataPropertyIRI1")] pub DataPropertyIRI,
    #[serde(rename = "dataPropertyIRI2")] pub DataPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl EquivalentDataProperties {
    pub fn data_property_iri_1(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn data_property_iri_2(&self) -> &DataPropertyIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
