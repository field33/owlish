use crate::owl::LiteralOrIRI;

use super::AnnotationPropertyIRI;
/// Annotations provide metadata to other concepts. They can be assigned to everything.
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Annotation {
    #[serde(rename = "annotationIRI")]
    pub iri: AnnotationPropertyIRI,
    #[serde(rename = "value")]
    pub value: LiteralOrIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Box<Annotation>>,
}

impl Annotation {
    pub fn new(
        iri: AnnotationPropertyIRI,
        value: LiteralOrIRI,
        annotations: Vec<Box<Annotation>>,
    ) -> Self {
        Self {
            iri,
            value,
            annotations,
        }
    }
}
