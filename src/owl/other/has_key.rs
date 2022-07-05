use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HasKey(pub ClassIRI, pub Vec<ObjectPropertyIRI>);
