use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectPropertyChain(pub Vec<ObjectPropertyIRI>);

impl From<ObjectPropertyChain> for ObjectPropertyConstructor {
    fn from(c: ObjectPropertyChain) -> Self {
        Self::ObjectPropertyChain(c)
    }
}
