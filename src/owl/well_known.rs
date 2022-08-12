use crate::owl::{AnnotationPropertyIRI, ClassIRI, DatatypeIRI, IRI};

// Datatypes

#[allow(non_snake_case)]
pub fn xsd_integer() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#integer")
        .unwrap()
        .into()
}

#[allow(non_snake_case)]
pub fn xsd_float() -> DatatypeIRI {
    IRI::new("http://www.w3.org/2001/XMLSchema#float")
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

#[allow(non_snake_case)]
pub fn owl_AnnotationProperty() -> ClassIRI {
    IRI::new("http://www.w3.org/2002/07/owl#AnnotationProperty")
        .unwrap()
        .into()
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * All well known OWL-2 relevant IRIs.
 */
export const well_known: {
    xsd_integer: IRI,
    xsd_nonNegativeInteger: IRI,
    xsd_minExclusive: IRI,
    xsd_minInclusive: IRI,
    xsd_maxInclusive: IRI,
    xsd_maxExclusive: IRI,
    rdfs_comment: IRI,
    rdfs_label: IRI,
    rdf_type: IRI,
    owl_Ontology: IRI,
    owl_Thing: IRI,
    owl_Class: IRI,
    owl_AsymmetricProperty: IRI,
    owl_SymmetricProperty: IRI,
    owl_ObjectProperty: IRI,
}
"#;
}
