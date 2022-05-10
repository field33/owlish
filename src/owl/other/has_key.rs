use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug)]
pub struct HasKey(pub(crate) ClassIRI, pub(crate) Vec<ObjectPropertyIRI>);
