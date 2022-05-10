mod axioms;
pub use axioms::*;

use crate::owl::IRI;

#[derive(Debug, Clone)]
pub struct IndividualIRI(IRI);
impl From<IRI> for IndividualIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
