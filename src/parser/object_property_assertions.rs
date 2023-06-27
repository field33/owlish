use super::collector::CollectedReification;
use super::collector::CollectedReificationKey;
use super::collector::MatcherHandler;
use crate::error::Error;
use crate::get_vars;
use crate::owl::well_known;
use crate::owl::Annotation;
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
                            match vars.object {
                                Value::Iri(iri) => {
                                    if let Ok(object) = IRI::new(iri) {
                                        if o.object_property_declaration(&predicate).is_some()
                                            || options.is_object_prop(&predicate)
                                        {
                                            let mut annotations = Vec::new();
                                            if let Some(CollectedReificationKey::Iri(iri)) = o
                                                .annotation_on_triple(&CollectedReification {
                                                    subject: subject.as_str().into(),
                                                    predicate: predicate.as_str().into(),
                                                    object: object.as_str().into(),
                                                })
                                            {
                                                if let Ok(iri) = IRI::new(iri) {
                                                    annotations.push(Annotation {
                                                        annotations: vec![],
                                                        iri: well_known::owl_annotatedSource()
                                                            .into(),
                                                        value: iri.into(),
                                                    })
                                                }
                                            }

                                            o.push_axiom(
                                                ObjectPropertyAssertion::new(
                                                    predicate.into(),
                                                    subject.into(),
                                                    object.into(),
                                                    annotations,
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
                                        if let Some(
                                            super::collector::CollectedBlankNode::Sequence {
                                                first,
                                                rest,
                                            },
                                        ) = o.get_blank(bn)
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
                                            )
                                            .into(),
                                        )
                                    }
                                }
                                _ => {
                                    //ignore
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
