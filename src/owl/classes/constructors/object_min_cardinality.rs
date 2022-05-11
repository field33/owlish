use crate::owl::{ClassIRI, ObjectPropertyIRI, ClassConstructor};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectMinCardinality(pub u64, pub ObjectPropertyIRI, pub Option<ClassIRI>);

impl From<ObjectMinCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectMinCardinality) -> Self {
        Box::new(ClassConstructor::ObjectMinCardinality(c))
    }
}
impl From<ObjectMinCardinality> for ClassConstructor {
    fn from(c: ObjectMinCardinality) -> Self {
        ClassConstructor::ObjectMinCardinality(c)
    }
}

impl ClassConstructor {
    pub fn object_min_cardinality(&self) -> Option<&ObjectMinCardinality> {
        match self {
            ClassConstructor::ObjectMinCardinality(d) => Some(d),
            _ => None,
        }
    }
}