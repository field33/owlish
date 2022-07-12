use crate::owl::{DataPropertyIRI, DatatypeIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyRange(pub DataPropertyIRI, pub DatatypeIRI);
