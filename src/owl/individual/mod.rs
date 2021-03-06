mod axioms;
pub use axioms::*;
use serde::{Deserialize, Serialize};

use crate::owl::IRI;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct IndividualIRI(IRI);

impl From<IRI> for IndividualIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl IndividualIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}
