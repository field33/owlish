use std::collections::HashMap;

use crate::{error::Error, owl::Declaration, parser::matcher::RdfMatcher, rdf_match};

use super::collector::{get_iri_var, MatcherHandler};

/// declarations
/// https://www.w3.org/TR/2012/REC-owl2-mapping-to-rdf-20121211/#Analyzing_Declarations
/// https://www.w3.org/TR/2012/REC-owl2-mapping-to-rdf-20121211/#Parsing_of_Axioms
pub(crate) fn match_declarations(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("Ontology", prefixes, [*:subject] [rdf:type] [owl:Ontology] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.set_iri(iri);
            }
            Ok(true)
        }),
    ));
    matchers.push((
        rdf_match!("Class", prefixes, [*:subject] [rdf:type] [owl:Class] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.push_declaration(Declaration::Class {
                    iri: iri.into(),
                    annotations: vec![],
                });
            }
            Ok(true)
        }),
    ));
    matchers.push((
        rdf_match!("Datatype",prefixes, [*:subject] [rdf:type] [rdfs:Datatype] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.push_declaration(Declaration::Datatype {
                    iri: iri.into(),
                    annotations: vec![],
                });
            }
            Ok(true)
        }),
    ));
    matchers.push((
        rdf_match!("ObjectProperty", prefixes, [*:subject] [rdf:type] [owl:ObjectProperty] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.push_declaration(Declaration::ObjectProperty {
                    iri: iri.into(),
                    annotations: vec![],
                });
            }
            Ok(true)
        }),
    ));
    matchers.push((
        rdf_match!("DataProperty", prefixes, [*:subject] [rdf:type] [owl:DatatypeProperty] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.push_declaration(Declaration::DataProperty {
                    iri: iri.into(),
                    annotations: vec![],
                });
            }
            Ok(true)
        }),
    ));
    matchers.push((
        rdf_match!("AnnotationProperty", prefixes, [*:subject] [rdf:type] [owl:AnnotationProperty] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.push_declaration(Declaration::AnnotationProperty{iri: iri.into(),annotations: vec![],});
            }
            Ok(true)
        }),
    ));
    matchers.push((
        rdf_match!("NamedIndividual", prefixes, [*:subject] [rdf:type] [owl:NamedIndividual] .)?,
        Box::new(|mstate, o, _| {
            if let Some(iri) = get_iri_var("subject", mstate)? {
                o.push_declaration(Declaration::NamedIndividual {
                    iri: iri.into(),
                    annotations: vec![],
                });
            }
            Ok(true)
        }),
    ));
    Ok(())
}
