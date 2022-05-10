use crate::owl::{AnnotationPropertyIRI, ClassIRI, DatatypeIRI, IRI};

// Datatypes

#[allow(non_snake_case)]
pub fn xsd_integer() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#integer")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn xsd_nonNegativeInteger() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#nonNegativeInteger")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn xsd_minExclusive() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#minExclusive")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn xsd_minInclusive() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#minInclusive")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn xsd_maxInclusive() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#maxInclusive")
        .unwrap()
        .into()
}
#[allow(non_snake_case)]
pub fn xsd_maxExclusive() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#maxExclusive")
        .unwrap()
        .into()
}

// Annotations

#[allow(non_snake_case)]
pub fn rdfs_comment() -> AnnotationPropertyIRI {
    IRI::new("http://www.w3.org/2000/01/rdf-schema#comment")
        .unwrap()
        .into()
}

// Classes

#[allow(non_snake_case)]
pub fn owl_Thing() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#Thing")
        .unwrap()
        .into()
}
