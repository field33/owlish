use crate::owl::IRI;

pub trait Regards {
    fn regards(&self, iri: &IRI) -> bool;
}
