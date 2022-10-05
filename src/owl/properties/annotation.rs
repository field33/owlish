use crate::owl::LiteralOrIRI;

use super::AnnotationPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Annotation {
    pub iri: AnnotationPropertyIRI,
    pub value: LiteralOrIRI,
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
