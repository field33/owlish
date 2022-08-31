use crate::owl::{Annotation, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReflexiveObjectProperty(
    #[serde(rename = "objectPropertyIRI")] pub ObjectPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl ReflexiveObjectProperty {
    pub fn object_property_iri(&self) -> &ObjectPropertyIRI {
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
export type ReflexiveObjectProperty = {
    objectPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
