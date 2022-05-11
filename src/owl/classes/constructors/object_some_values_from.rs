use crate::owl::{ObjectPropertyConstructor, ClassIRI, Annotation, ClassConstructor};

/// Class construction based on properties.
#[derive(Debug, Eq, PartialEq)]
pub struct ObjectSomeValuesFrom(
    pub ObjectPropertyConstructor,
    pub ClassIRI,
    pub Vec<Annotation>,
);
impl From<ObjectSomeValuesFrom> for ClassConstructor {
    fn from(c: ObjectSomeValuesFrom) -> Self {
        ClassConstructor::ObjectSomeValuesFrom(c)
    }
}

impl From<ObjectSomeValuesFrom> for Box<ClassConstructor> {
    fn from(c: ObjectSomeValuesFrom) -> Self {
        Box::new(ClassConstructor::ObjectSomeValuesFrom(c))
    }
}

impl ClassConstructor {
    pub fn object_some_values_from(&self) -> Option<&ObjectSomeValuesFrom> {
        match self {
            ClassConstructor::ObjectSomeValuesFrom(d) => Some(d),
            _ => None,
        }
    }
}
