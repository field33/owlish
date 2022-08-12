use crate::owl::LiteralOrIRI;

use super::AnnotationPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Annotation(
    pub AnnotationPropertyIRI,
    pub LiteralOrIRI,
    pub Vec<Box<Annotation>>,
);

impl Annotation {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn value(&self) -> &LiteralOrIRI {
        &self.1
    }
    pub fn annotations(&self) -> Vec<&Annotation> {
        self.2.iter().map(|ba| ba.as_ref()).collect()
    }
}
