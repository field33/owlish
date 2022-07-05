use crate::owl::{Annotation, ClassConstructor, Regards, IRI, Axiom};

/// Defines that the subject is a sub class of the object.
///
/// Structure `(subject, object, annotations)`.
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubClassOf(
    /// subject
    pub Box<ClassConstructor>,
    /// object
    pub Box<ClassConstructor>,
    pub Vec<Annotation>,
);
impl SubClassOf {
    pub fn subject(&self) -> &ClassConstructor {
        &self.0
    }
    pub fn parent(&self) -> &ClassConstructor {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

impl From<SubClassOf> for Box<ClassConstructor> {
    fn from(sco: SubClassOf) -> Self {
        Box::new(ClassConstructor::SubClassOf(sco))
    }
}
impl From<SubClassOf> for ClassConstructor {
    fn from(sco: SubClassOf) -> Self {
        ClassConstructor::SubClassOf(sco)
    }
}
// impl<'a> From<&'a SubClassOf> for &'a ClassConstructor {
//     fn from(sco: &'a SubClassOf) -> Self {
//         &ClassConstructor::SubClassOf(*sco)
//     }
// }

impl Regards for SubClassOf {
    fn regards(&self, iri: &IRI) -> bool {
        self.subject().regards(iri) || self.parent().regards(iri)
    }
}

impl ClassConstructor {
    pub fn sub_class_of(&self) -> Option<&SubClassOf> {
        match self {
            ClassConstructor::SubClassOf(s) => Some(s),
            _ => None,
        }
    }
}

impl From<SubClassOf> for Axiom {
    fn from(sco: SubClassOf) -> Self {
        Axiom::SubClassOf(sco)
    }
}
