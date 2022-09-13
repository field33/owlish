use crate::owl::{AnnotationPropertyIRI, ClassIRI, DatatypeIRI, IRI};

// Datatypes
#[allow(non_upper_case_globals)]
pub const xsd_base_str: &str = "http://www.w3.org/2001/XMLSchema#";

#[allow(non_upper_case_globals)]
pub const xsd_string_str: &str = "http://www.w3.org/2001/XMLSchema#string";

#[allow(non_snake_case)]
pub fn xsd_string() -> DatatypeIRI {
    IRI::new(xsd_string_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_dateTime_str: &str = "http://www.w3.org/2001/XMLSchema#dateTime";

#[allow(non_snake_case)]
pub fn xsd_dateTime() -> DatatypeIRI {
    IRI::new(xsd_dateTime_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_boolean_str: &str = "http://www.w3.org/2001/XMLSchema#boolean";

#[allow(non_snake_case)]
pub fn xsd_boolean() -> DatatypeIRI {
    IRI::new(xsd_boolean_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_integer_str: &str = "http://www.w3.org/2001/XMLSchema#integer";

#[allow(non_snake_case)]
pub fn xsd_integer() -> DatatypeIRI {
    IRI::new(xsd_integer_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_float_str: &str = "http://www.w3.org/2001/XMLSchema#float";

#[allow(non_snake_case)]
pub fn xsd_float() -> DatatypeIRI {
    IRI::new(xsd_float_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_nonNegativeInteger_str: &str = "http://www.w3.org/2001/XMLSchema#nonNegativeInteger";

#[allow(non_snake_case)]
pub fn xsd_nonNegativeInteger() -> DatatypeIRI {
    IRI::new(xsd_nonNegativeInteger_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_minExclusive_str: &str = "http://www.w3.org/2001/XMLSchema#minExclusive";

#[allow(non_snake_case)]
pub fn xsd_minExclusive() -> DatatypeIRI {
    IRI::new(xsd_minExclusive_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_minInclusive_str: &str = "http://www.w3.org/2001/XMLSchema#minInclusive";

#[allow(non_snake_case)]
pub fn xsd_minInclusive() -> DatatypeIRI {
    IRI::new(xsd_minInclusive_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_maxInclusive_str: &str = "http://www.w3.org/2001/XMLSchema#maxInclusive";

#[allow(non_snake_case)]
pub fn xsd_maxInclusive() -> DatatypeIRI {
    IRI::new(xsd_maxInclusive_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_maxExclusive_str: &str = "http://www.w3.org/2001/XMLSchema#maxExclusive";

#[allow(non_snake_case)]
pub fn xsd_maxExclusive() -> DatatypeIRI {
    IRI::new(xsd_maxExclusive_str).unwrap().into()
}

// RDFS
#[allow(non_upper_case_globals)]
pub const rdfs_base_str: &str = "http://www.w3.org/2000/01/rdf-schema#";

#[allow(non_upper_case_globals)]
pub const rdfs_comment_str: &str = "http://www.w3.org/2000/01/rdf-schema#comment";

#[allow(non_snake_case)]
pub fn rdfs_comment() -> AnnotationPropertyIRI {
    IRI::new(rdfs_comment_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const rdfs_label_str: &str = "http://www.w3.org/2000/01/rdf-schema#label";

#[allow(non_snake_case)]
pub fn rdfs_label() -> AnnotationPropertyIRI {
    IRI::new(rdfs_label_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const rdfs_subClassOf_str: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";

#[allow(non_snake_case)]
pub fn rdfs_subClassOf() -> AnnotationPropertyIRI {
    IRI::new(rdfs_subClassOf_str).unwrap().into()
}


// RDF
#[allow(non_upper_case_globals)]
pub const rdf_base_str: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

#[allow(non_upper_case_globals)]
pub const rdf_type_str: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

#[allow(non_snake_case)]
pub fn rdf_type() -> AnnotationPropertyIRI {
    IRI::new(rdf_type_str).unwrap().into()
}

// OWL
#[allow(non_upper_case_globals)]
pub const owl_base_str: &str = "http://www.w3.org/2002/07/owl#";

#[allow(non_upper_case_globals)]
pub const owl_Ontology_str: &str = "http://www.w3.org/2002/07/owl#Ontology";

#[allow(non_snake_case)]
pub fn owl_Ontology() -> IRI {
    IRI::new(owl_Ontology_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_Thing_str: &str = "http://www.w3.org/2002/07/owl#Thing";

#[allow(non_snake_case)]
pub fn owl_Thing() -> ClassIRI {
    IRI::new(owl_Thing_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_Class_str: &str = "http://www.w3.org/2002/07/owl#Class";

#[allow(non_snake_case)]
pub fn owl_Class() -> ClassIRI {
    IRI::new(owl_Class_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_AsymmetricProperty_str: &str = "http://www.w3.org/2002/07/owl#AsymmetricProperty";

#[allow(non_snake_case)]
pub fn owl_AsymmetricProperty() -> ClassIRI {
    IRI::new(owl_AsymmetricProperty_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_SymmetricProperty_str: &str = "http://www.w3.org/2002/07/owl#SymmetricProperty";

#[allow(non_snake_case)]
pub fn owl_SymmetricProperty() -> ClassIRI {
    IRI::new(owl_SymmetricProperty_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_ObjectProperty_str: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";

#[allow(non_snake_case)]
pub fn owl_ObjectProperty() -> ClassIRI {
    IRI::new(owl_ObjectProperty_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_AnnotationProperty_str: &str = "http://www.w3.org/2002/07/owl#AnnotationProperty";

#[allow(non_snake_case)]
pub fn owl_AnnotationProperty() -> ClassIRI {
    IRI::new(owl_AnnotationProperty_str).unwrap().into()
}

#[cfg(test)]
mod test {

    #[test]
    fn well_known() {
        super::owl_Ontology();
    }
}
