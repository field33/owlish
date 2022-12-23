use crate::owl::{AnnotationPropertyIRI, ClassIRI, DatatypeIRI, IRI};

use super::IndividualIRI;

// Datatypes
#[allow(non_upper_case_globals)]
pub const xsd_base_str: &str = "http://www.w3.org/2001/XMLSchema#";

#[allow(non_snake_case)]
pub fn xsd() -> IRI {
    IRI::new(xsd_base_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const xsd_string_str: &str = "http://www.w3.org/2001/XMLSchema#string";

#[allow(non_snake_case)]
pub fn xsd_string() -> DatatypeIRI {
    IRI::new(xsd_string_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const xsd_decimal_str: &str = "http://www.w3.org/2001/XMLSchema#decimal";

#[allow(non_snake_case)]
pub fn xsd_decimal() -> DatatypeIRI {
    IRI::new(xsd_decimal_str).unwrap().into()
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
#[allow(non_snake_case)]
pub fn rdfs() -> IRI {
    IRI::new(rdfs_base_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const rdfs_Datatype_str: &str = "http://www.w3.org/2000/01/rdf-schema#Datatype";
#[allow(non_snake_case)]
pub fn rdfs_Datatype() -> IRI {
    IRI::new(rdfs_Datatype_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const rdfs_domain_str: &str = "http://www.w3.org/2000/01/rdf-schema#domain";
#[allow(non_snake_case)]
pub fn rdfs_domain() -> IRI {
    IRI::new(rdfs_domain_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const rdfs_range_str: &str = "http://www.w3.org/2000/01/rdf-schema#range";
#[allow(non_snake_case)]
pub fn rdfs_range() -> IRI {
    IRI::new(rdfs_range_str).unwrap()
}


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

#[allow(non_snake_case)]
pub fn rdf() -> IRI {
    IRI::new(rdf_base_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const rdf_first_str: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
#[allow(non_snake_case)]
pub fn rdf_first() -> IRI {
    IRI::new(rdf_first_str).unwrap()
}
#[allow(non_upper_case_globals)]
pub const rdf_rest_str: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
#[allow(non_snake_case)]
pub fn rdf_rest() -> IRI {
    IRI::new(rdf_rest_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const rdf_type_str: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

#[allow(non_snake_case)]
pub fn rdf_type() -> AnnotationPropertyIRI {
    IRI::new(rdf_type_str).unwrap().into()
}

// OWL
#[allow(non_upper_case_globals)]
pub const owl_base_str: &str = "http://www.w3.org/2002/07/owl#";

#[allow(non_snake_case)]
pub fn owl() -> IRI {
    IRI::new(owl_base_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_oneOf_str: &str = "http://www.w3.org/2002/07/owl#oneOf";
#[allow(non_snake_case)]
pub fn owl_oneOf() -> IRI {
    IRI::new(owl_oneOf_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_onDatatype_str: &str = "http://www.w3.org/2002/07/owl#onDatatype";
#[allow(non_snake_case)]
pub fn owl_onDatatype() -> IRI {
    IRI::new(owl_onDatatype_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_withRestrictions_str: &str = "http://www.w3.org/2002/07/owl#withRestrictions";
#[allow(non_snake_case)]
pub fn owl_withRestrictions() -> IRI {
    IRI::new(owl_withRestrictions_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_inverseOf_str: &str = "http://www.w3.org/2002/07/owl#inverseOf";
#[allow(non_snake_case)]
pub fn owl_inverseOf() -> IRI {
    IRI::new(owl_inverseOf_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_someValuesFrom_str: &str = "http://www.w3.org/2002/07/owl#someValuesFrom";
#[allow(non_snake_case)]
pub fn owl_someValuesFrom() -> IRI {
    IRI::new(owl_someValuesFrom_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_onProperties_str: &str = "http://www.w3.org/2002/07/owl#onProperties";
#[allow(non_snake_case)]
pub fn owl_onProperties() -> IRI {
    IRI::new(owl_onProperties_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_onDataRange_str: &str = "http://www.w3.org/2002/07/owl#onDataRange";
#[allow(non_snake_case)]
pub fn owl_onDataRange() -> IRI {
    IRI::new(owl_onDataRange_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_allValuesFrom_str: &str = "http://www.w3.org/2002/07/owl#allValuesFrom";
#[allow(non_snake_case)]
pub fn owl_allValuesFrom() -> IRI {
    IRI::new(owl_allValuesFrom_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_hasValue_str: &str = "http://www.w3.org/2002/07/owl#hasValue";
#[allow(non_snake_case)]
pub fn owl_hasValue() -> IRI {
    IRI::new(owl_hasValue_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_hasSelf_str: &str = "http://www.w3.org/2002/07/owl#hasSelf";
#[allow(non_snake_case)]
pub fn owl_hasSelf() -> IRI {
    IRI::new(owl_hasSelf_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_minCardinality_str: &str = "http://www.w3.org/2002/07/owl#minCardinality";
#[allow(non_snake_case)]
pub fn owl_minCardinality() -> IRI {
    IRI::new(owl_minCardinality_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_maxCardinality_str: &str = "http://www.w3.org/2002/07/owl#maxCardinality";
#[allow(non_snake_case)]
pub fn owl_maxCardinality() -> IRI {
    IRI::new(owl_maxCardinality_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_maxQualifiedCardinality_str: &str =
    "http://www.w3.org/2002/07/owl#maxQualifiedCardinality";
#[allow(non_snake_case)]
pub fn owl_maxQualifiedCardinality() -> IRI {
    IRI::new(owl_maxQualifiedCardinality_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_minQualifiedCardinality_str: &str =
    "http://www.w3.org/2002/07/owl#minQualifiedCardinality";
#[allow(non_snake_case)]
pub fn owl_minQualifiedCardinality() -> IRI {
    IRI::new(owl_minQualifiedCardinality_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_cardinality_str: &str = "http://www.w3.org/2002/07/owl#cardinality";
#[allow(non_snake_case)]
pub fn owl_cardinality() -> IRI {
    IRI::new(owl_cardinality_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_qualifiedCardinality_str: &str = "http://www.w3.org/2002/07/owl#qualifiedCardinality";
#[allow(non_snake_case)]
pub fn owl_qualifiedCardinality() -> IRI {
    IRI::new(owl_qualifiedCardinality_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_onProperty_str: &str = "http://www.w3.org/2002/07/owl#onProperty";
#[allow(non_snake_case)]
pub fn owl_onProperty() -> IRI {
    IRI::new(owl_onProperty_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_intersectionOf_str: &str = "http://www.w3.org/2002/07/owl#intersectionOf";
#[allow(non_snake_case)]
pub fn owl_intersectionOf() -> IRI {
    IRI::new(owl_intersectionOf_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_unionOf_str: &str = "http://www.w3.org/2002/07/owl#unionOf";
#[allow(non_snake_case)]
pub fn owl_unionOf() -> IRI {
    IRI::new(owl_unionOf_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_complementOf_str: &str = "http://www.w3.org/2002/07/owl#complementOf";
#[allow(non_snake_case)]
pub fn owl_complementOf() -> IRI {
    IRI::new(owl_complementOf_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_onClass_str: &str = "http://www.w3.org/2002/07/owl#onClass";
#[allow(non_snake_case)]
pub fn owl_onClass() -> IRI {
    IRI::new(owl_onClass_str).unwrap()
}

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
pub const owl_Restriction_str: &str = "http://www.w3.org/2002/07/owl#Restriction";
#[allow(non_snake_case)]
pub fn owl_Restriction() -> ClassIRI {
    IRI::new(owl_Restriction_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_Class_str: &str = "http://www.w3.org/2002/07/owl#Class";
#[allow(non_snake_case)]
pub fn owl_Class() -> ClassIRI {
    IRI::new(owl_Class_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_NamedIndividual_str: &str = "http://www.w3.org/2002/07/owl#NamedIndividual";
#[allow(non_snake_case)]
pub fn owl_NamedIndividual() -> IndividualIRI {
    IRI::new(owl_Class_str).unwrap().into()
}

#[allow(non_upper_case_globals)]
pub const owl_AsymmetricProperty_str: &str = "http://www.w3.org/2002/07/owl#AsymmetricProperty";
#[allow(non_snake_case)]
pub fn owl_AsymmetricProperty() -> IRI {
    IRI::new(owl_AsymmetricProperty_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_SymmetricProperty_str: &str = "http://www.w3.org/2002/07/owl#SymmetricProperty";
#[allow(non_snake_case)]
pub fn owl_SymmetricProperty() -> IRI {
    IRI::new(owl_SymmetricProperty_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_ObjectProperty_str: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
#[allow(non_snake_case)]
pub fn owl_ObjectProperty() -> IRI {
    IRI::new(owl_ObjectProperty_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_DatatypeProperty_str: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
#[allow(non_snake_case)]
pub fn owl_DatatypeProperty() -> IRI {
    IRI::new(owl_DatatypeProperty_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_AnnotationProperty_str: &str = "http://www.w3.org/2002/07/owl#AnnotationProperty";
#[allow(non_snake_case)]
pub fn owl_AnnotationProperty() -> IRI {
    IRI::new(owl_AnnotationProperty_str).unwrap()
}

#[allow(non_upper_case_globals)]
pub const owl_Datatype_str: &str = "http://www.w3.org/2002/07/owl#Datatype";
#[allow(non_snake_case)]
pub fn owl_Datatype() -> IRI {
    IRI::new(owl_Datatype_str).unwrap()
}

#[cfg(test)]
mod test {

    #[test]
    fn well_known() {
        super::owl_Ontology();
    }
}
