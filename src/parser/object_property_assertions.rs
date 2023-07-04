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
            let Some(vars) = get_vars!(mstate, subject, predicate, object) else {
                return Ok(false);
            };
            let Value::Iri(iri) = vars.subject else {
                return Ok(false);
            };
            let Ok(subject) = IRI::new(iri) else {
                return Ok(false);
            };
            let Ok(predicate) = vars.predicate.clone().try_into() else {
                return Ok(false);
            };

            match vars.object {
                Value::Iri(iri) => {
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
                                    vec![]
                                )
                                .into(),
                            )
                        }
                    }
                }
                Value::Blank(bn) => {
                    let mut object = Vec::new();
                    let mut b = Some(bn);
                    while let Some(bn) = b {
                        b = None;
                        if let Some(super::collector::CollectedBlankNode::Sequence {
                            first,
                            rest,
                        }) = o.get_blank(bn)
                        {
                            if let Some(Value::Iri(iri)) = first {
                                if let Ok(iri) = IRI::new(iri) {
                                    object.push(iri);
                                    b = rest.as_ref();
                                }
                            }
                        }
                    }
                    if object.len() > 0 {
                        o.push_axiom(
                            ObjectPropertyAssertion::new_with_list(
                                predicate.into(),
                                subject.into(),
                                object,
                                vec![],
                                vec![],
                            )
                            .into(),
                        )
                    }
                }
                Value::Literal { .. } => {
                    unreachable!("Branch should be unreachable, as matcher shouldn't match literal objects.")
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}
