use crate::owl::{AnnotationAssertion, LiteralOrIRI, ResourceId};

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

    /// Turn an annotation into an AnnotationAssertion by combining it with a subject.
    ///
    /// This slightly breaks with the semantic difference between `Annotation` and `AnnotationAssertion`,
    /// as many `Annotation`s in the OWL sense are not "assertions".
    /// However this provides an easier to use interface, especially when it comes to reifications.
    pub fn to_assertion(self, subject_resource_id: ResourceId) -> AnnotationAssertion {
        AnnotationAssertion::new(self.iri, subject_resource_id, self.value, self.annotations.clone().into_iter().map(|n| *n).collect(), vec![])
    }
}
