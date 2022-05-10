use crate::owl::{Annotation, ClassConstructor, Regards, IRI};

#[derive(Debug)]
pub struct SubClassOf(
    pub(crate) Box<ClassConstructor>,
    pub(crate) Box<ClassConstructor>,
    pub(crate) Vec<Annotation>,
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

impl Regards for SubClassOf {
    fn regards(&self, iri: &IRI) -> bool {
        self.subject().regards(iri) || self.parent().regards(iri)
    }
}
