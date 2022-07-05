use crate::owl::ObjectPropertyIRI;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EquivalentObjectProperties(pub ObjectPropertyIRI, pub ObjectPropertyIRI);
