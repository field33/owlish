use crate::owl::{Annotation, ClassConstructor, ClassIRI};

#[derive(Debug, Eq, PartialEq)]
pub struct EquivalentClasses(
    pub(crate) ClassIRI,
    pub(crate) Box<ClassConstructor>,
    pub(crate) Vec<Annotation>,
);
impl From<EquivalentClasses> for Box<ClassConstructor> {
    fn from(c: EquivalentClasses) -> Self {
        Box::new(ClassConstructor::EquivalentClasses(c))
    }
}
impl From<EquivalentClasses> for ClassConstructor {
    fn from(c: EquivalentClasses) -> Self {
        ClassConstructor::EquivalentClasses(c)
    }
}

impl ClassConstructor {
    pub fn equivalent_classes(&self) -> Option<&EquivalentClasses> {
        match self {
            ClassConstructor::EquivalentClasses(d) => Some(d),
            _ => None,
        }
    }
}