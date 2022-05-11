use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug)]
pub struct HasKey(pub ClassIRI, pub Vec<ObjectPropertyIRI>);
