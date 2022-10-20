use super::collector::MatcherHandler;
use crate::error::Error;
use crate::get_vars;
use crate::owl::ObjectPropertyAssertion;
use crate::owl::IRI;
use crate::parser::matcher::RdfMatcher;
use crate::parser::matcher::Value;
use crate::rdf_match;
use std::collections::HashMap;

// const WELL_KNOWN_OBJECT_PROPERTIES: [&str; 2] = [
//     well_known::rdfs_subClassOf_str,
//     //
// ];

pub(crate) fn push(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    _prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("ObjectPropertyAssertion", _prefixes,
            // [iri or blank] [iri] [iri or blank]
            [+:subject] [*:predicate] [+:object] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(vars) = get_vars!(mstate, subject, predicate, object) {
                if let Ok(predicate) = vars.predicate.clone().try_into() {
                    let predicate: IRI = predicate;
                    if let Value::Iri(iri) = vars.subject {
                        if let Ok(subject) = IRI::new(iri) {
                            if let Value::Iri(iri) = vars.object {
                                if let Ok(object) = IRI::new(iri) {
                                    if o.object_property_declaration(&predicate).is_some()
                                        || options.is_object_prop(&predicate)
                                    {
                                        o.push_axiom(
                                            ObjectPropertyAssertion::new(
                                                predicate.into(),
                                                subject.into(),
                                                object.into(),
                                                vec![],
                                            )
                                            .into(),
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}
