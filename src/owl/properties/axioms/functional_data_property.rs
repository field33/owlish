use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FunctionalDataProperty(
    #[serde(rename = "dataPropertyIRI")] pub DataPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl FunctionalDataProperty {
    pub fn data_property_iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.1
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type FunctionalDataProperty = {
    dataPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
