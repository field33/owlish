use crate::owl::{Annotation, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HasKey(
    #[serde(rename = "iri")] pub ClassIRI,
    #[serde(rename = "objectProperties")] pub Vec<ObjectPropertyIRI>,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type HasKey = {
    iri: IRI, 
    objectProperties: Array<IRI>,
    annotations: Array<Annotation>,
}
"#;
}
