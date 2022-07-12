use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HasKey(pub ClassIRI, pub Vec<ObjectPropertyIRI>);
