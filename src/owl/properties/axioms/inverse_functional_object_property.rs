use crate::owl::ObjectPropertyIRI;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InverseFunctionalObjectProperty(pub ObjectPropertyIRI);
