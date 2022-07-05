use crate::owl::{ClassIRI, ObjectPropertyIRI, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectMaxCardinality(pub u64, pub ObjectPropertyIRI, pub Option<ClassIRI>);

impl From<ObjectMaxCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectMaxCardinality) -> Self {
        Box::new(ClassConstructor::ObjectMaxCardinality(c))
    }
}
impl From<ObjectMaxCardinality> for ClassConstructor {
    fn from(c: ObjectMaxCardinality) -> Self {
        ClassConstructor::ObjectMaxCardinality(c)
    }
}

impl ClassConstructor {
    pub fn object_max_cardinality(&self) -> Option<&ObjectMaxCardinality> {
        match self {
            ClassConstructor::ObjectMaxCardinality(d) => Some(d),
            _ => None,
        }
    }
}
