use crate::owl::{DatatypeIRI, Regards, Value};

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Eq, PartialEq)]
pub enum Restriction {
    Numeric(DatatypeIRI, Value),
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatatypeRestriction(pub(crate) DatatypeIRI, pub(crate) Vec<Restriction>);

impl From<DatatypeRestriction> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DatatypeRestriction) -> Self {
        DatatypeDefinitionConstructor::DatatypeRestriction(c).into()
    }
}
impl From<DatatypeRestriction> for DatatypeDefinitionConstructor {
    fn from(c: DatatypeRestriction) -> Self {
        DatatypeDefinitionConstructor::DatatypeRestriction(c)
    }
}

impl DatatypeRestriction {
    pub fn datatype_iri(&self) -> &DatatypeIRI {
        &self.0
    }
    pub fn restrictions(&self) -> &Vec<Restriction> {
        &self.1
    }
}

impl Regards for DatatypeRestriction {
    fn regards(&self, iri: &crate::owl::IRI) -> bool {
        self.datatype_iri().as_iri() == iri
            || self.restrictions().iter().any(|r| match r {
                Restriction::Numeric(d, _) => d.as_iri() == iri,
            })
    }
}
