use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectInverseOf(pub ObjectPropertyIRI);

impl From<ObjectInverseOf> for ObjectPropertyConstructor {
    fn from(c: ObjectInverseOf) -> Self {
        Self::ObjectInverseOf(c)
    }
}
