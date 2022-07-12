use crate::owl::{Axiom, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SymmetricObjectProperty(pub ObjectPropertyIRI);

impl From<SymmetricObjectProperty> for Axiom {
    fn from(sop: SymmetricObjectProperty) -> Self {
        Self::SymmetricObjectProperty(sop)
    }
}
