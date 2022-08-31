use crate::owl::{Annotation, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentObjectProperties(
    #[serde(rename = "objectPropertyIRI1")] pub ObjectPropertyIRI,
    #[serde(rename = "objectPropertyIRI2")] pub ObjectPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl EquivalentObjectProperties {
    pub fn subject(&self) -> &ObjectPropertyIRI {
        &self.0
    }
    pub fn object(&self) -> &ObjectPropertyIRI {
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
export type EquivalentObjectProperties = {
    objectPropertyIRI1: IRI,
    objectPropertyIRI2: IRI,
    annotations: Array<Annotation>,
};
"#;
}
