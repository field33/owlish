use crate::owl::ObjectPropertyIRI;

#[derive(Debug)]
pub struct DisjointObjectProperties(pub(crate) ObjectPropertyIRI, pub(crate) ObjectPropertyIRI);
