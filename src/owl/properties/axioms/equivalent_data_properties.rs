use crate::owl::DataPropertyIRI;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EquivalentDataProperties(pub DataPropertyIRI, pub DataPropertyIRI);
