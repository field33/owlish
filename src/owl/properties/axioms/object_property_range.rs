use crate::owl::{Axiom, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyRange(pub ObjectPropertyIRI, pub ClassIRI);

impl From<ObjectPropertyRange> for Axiom {
    fn from(opr: ObjectPropertyRange) -> Self {
        Axiom::ObjectPropertyRange(opr)
    }
}
