use crate::owl::DataPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentDataProperties(pub DataPropertyIRI, pub DataPropertyIRI);
