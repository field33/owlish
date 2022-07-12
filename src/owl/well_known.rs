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

// RDFS

#[allow(non_snake_case)]
pub fn rdfs_comment() -> AnnotationPropertyIRI {
    IRI::new("http://www.w3.org/2000/01/rdf-schema#comment")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn rdfs_label() -> AnnotationPropertyIRI {
    IRI::new("http://www.w3.org/2000/01/rdf-schema#label")
        .unwrap()
        .into()
}

// RDF

#[allow(non_snake_case)]
pub fn rdf_type() -> AnnotationPropertyIRI {
    IRI::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
        .unwrap()
        .into()
}

// OWL

#[allow(non_snake_case)]
pub fn owl_Ontology() -> IRI {
    IRI::new("http://www.w3.org/2002/07/owl#Ontology").unwrap()
}

#[allow(non_snake_case)]
pub fn owl_Thing() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#Thing")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn owl_Class() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#Class")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn owl_AsymmetricProperty() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#AsymmetricProperty")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn owl_SymmetricProperty() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#SymmetricProperty")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn owl_ObjectProperty() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#ObjectProperty")
        .unwrap()
        .into()
}
