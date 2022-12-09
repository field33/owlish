use crate::owl::{Annotation, AnnotationPropertyIRI};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubAnnotationPropertyOf {
    #[serde(rename = "annotationPropertyIRI")]
    pub subject_iri: AnnotationPropertyIRI,
    #[serde(rename = "parentAnnotationPropertyIRI")]
    pub parent_iri: AnnotationPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl SubAnnotationPropertyOf {
    pub fn new(
        subject_iri: AnnotationPropertyIRI,
        parent_iri: AnnotationPropertyIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            subject_iri,
            parent_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type SubAnnotationProperty = {
    annotationPropertyIRI: IRI,
    parentAnnotationPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
