use crate::owl::{ObjectPropertyIRI, Axiom};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AsymmetricObjectProperty(pub ObjectPropertyIRI);

impl From<AsymmetricObjectProperty> for Axiom {
    fn from(aop: AsymmetricObjectProperty) -> Self {
        Self::AsymmetricObjectProperty(aop)
    }
}