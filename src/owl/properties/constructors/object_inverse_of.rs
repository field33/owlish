use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectInverseOf(pub ObjectPropertyIRI);

impl From<ObjectInverseOf> for ObjectPropertyConstructor {
    fn from(c: ObjectInverseOf) -> Self {
        Self::ObjectInverseOf(c)
    }
}
