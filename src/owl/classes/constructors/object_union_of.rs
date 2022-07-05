use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectUnionOf(pub(crate) Vec<ClassConstructor>, pub(crate) Vec<Annotation>);
impl From<ObjectUnionOf> for ClassConstructor {
    fn from(c: ObjectUnionOf) -> Self {
        ClassConstructor::ObjectUnionOf(c)
    }
}
impl From<ObjectUnionOf> for Box<ClassConstructor> {
    fn from(c: ObjectUnionOf) -> Self {
        Box::new(ClassConstructor::ObjectUnionOf(c))
    }
}

impl ClassConstructor {
    pub fn object_union_of(&self) -> Option<&ObjectUnionOf> {
        match self {
            ClassConstructor::ObjectUnionOf(d) => Some(d),
            _ => None,
        }
    }
}
