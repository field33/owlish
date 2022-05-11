use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectComplementOf(pub(crate) Box<ClassConstructor>, pub(crate) Vec<Annotation>);

impl From<ObjectComplementOf> for Box<ClassConstructor> {
    fn from(c: ObjectComplementOf) -> Self {
        Box::new(ClassConstructor::ObjectComplementOf(c))
    }
}
impl From<ObjectComplementOf> for ClassConstructor {
    fn from(c: ObjectComplementOf) -> Self {
        ClassConstructor::ObjectComplementOf(c)
    }
}

impl ClassConstructor {
    pub fn object_complement_of(&self) -> Option<&ObjectComplementOf> {
        match self {
            ClassConstructor::ObjectComplementOf(d) => Some(d),
            _ => None,
        }
    }
}
