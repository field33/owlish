use crate::owl::ObjectPropertyIRI;

#[derive(Debug)]
pub struct InverseObjectProperties(pub(crate) ObjectPropertyIRI, pub(crate) ObjectPropertyIRI);
