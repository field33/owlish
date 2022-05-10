use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug)]
pub struct ObjectInverseOf(pub(crate) ObjectPropertyIRI);

impl From<ObjectInverseOf> for ObjectPropertyConstructor {
    fn from(c: ObjectInverseOf) -> Self {
        Self::ObjectInverseOf(c)
    }
}
