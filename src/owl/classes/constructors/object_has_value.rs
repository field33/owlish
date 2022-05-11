use crate::owl::{Annotation, ObjectPropertyConstructor, Value, ClassConstructor};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectHasValue(
    pub ObjectPropertyConstructor,
    pub Value,
    pub Vec<Annotation>,
);

impl From<ObjectHasValue> for Box<ClassConstructor> {
    fn from(c: ObjectHasValue) -> Self {
        Box::new(ClassConstructor::ObjectHasValue(c))
    }
}
impl From<ObjectHasValue> for ClassConstructor {
    fn from(c: ObjectHasValue) -> Self {
        ClassConstructor::ObjectHasValue(c)
    }
}

impl ClassConstructor {
    pub fn object_has_value(&self) -> Option<&ObjectHasValue> {
        match self {
            ClassConstructor::ObjectHasValue(d) => Some(d),
            _ => None,
        }
    }
}
