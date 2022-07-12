use crate::owl::ObjectPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransitiveObjectProperty(pub ObjectPropertyIRI);
