use crate::owl::{Axiom, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AsymmetricObjectProperty(pub ObjectPropertyIRI);

impl From<AsymmetricObjectProperty> for Axiom {
    fn from(aop: AsymmetricObjectProperty) -> Self {
        Self::AsymmetricObjectProperty(aop)
    }
}
