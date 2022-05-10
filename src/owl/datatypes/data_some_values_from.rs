use crate::owl::{DataPropertyIRI, DatatypeRestriction};

#[derive(Debug)]
pub struct DataSomeValuesFrom(pub(crate) DataPropertyIRI, pub(crate) DatatypeRestriction);
