use crate::owl::{Axiom, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyDomain(pub ObjectPropertyIRI, pub ClassIRI);

impl From<ObjectPropertyDomain> for Axiom {
    fn from(opd: ObjectPropertyDomain) -> Self {
        Axiom::ObjectPropertyDomain(opd)
    }
}
