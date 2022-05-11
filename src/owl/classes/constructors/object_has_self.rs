use crate::owl::{ObjectPropertyConstructor, Annotation, ClassConstructor};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectHasSelf(pub ObjectPropertyConstructor, pub Vec<Annotation>);

impl From<ObjectHasSelf> for Box<ClassConstructor> {
    fn from(c: ObjectHasSelf) -> Self {
        Box::new(ClassConstructor::ObjectHasSelf(c))
    }
}
impl From<ObjectHasSelf> for ClassConstructor {
    fn from(c: ObjectHasSelf) -> Self {
        ClassConstructor::ObjectHasSelf(c)
    }
}

impl ClassConstructor {
    pub fn object_has_self(&self) -> Option<&ObjectHasSelf> {
        match self {
            ClassConstructor::ObjectHasSelf(d) => Some(d),
            _ => None,
        }
    }
}
