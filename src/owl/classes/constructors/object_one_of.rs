use crate::owl::{Annotation, ClassConstructor, IndividualIRI};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectOneOf(pub Vec<IndividualIRI>, pub Vec<Annotation>);

impl From<ObjectOneOf> for Box<ClassConstructor> {
    fn from(c: ObjectOneOf) -> Self {
        Box::new(ClassConstructor::ObjectOneOf(c))
    }
}

impl From<ObjectOneOf> for ClassConstructor {
    fn from(c: ObjectOneOf) -> Self {
        ClassConstructor::ObjectOneOf(c)
    }
}

impl ClassConstructor {
    pub fn object_one_of(&self) -> Option<&ObjectOneOf> {
        match self {
            ClassConstructor::ObjectOneOf(d) => Some(d),
            _ => None,
        }
    }
}
