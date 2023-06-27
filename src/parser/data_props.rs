use std::{collections::HashMap, convert::TryInto};

use crate::{
    error::Error,
    owl::{Annotation, DataPropertyAssertion, Literal, IRI},
    parser::matcher::{RdfMatcher, Value},
    rdf_match,
};

use super::{
    collector::{get_iri_var, CollectedReificationKey, MatcherHandler, OntologyCollector},
    matcher::MatcherState,
};

/// simple dataprop assertions without blank nodes
pub(crate) fn match_simple_dataprop_assertions<'a>(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler<'a>)>,
    _prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("DataPropertyAssertionSimple", _prefixes, 
            [iob:subject] [*:predicate] [lt:object] .)?,
        Box::new(|mstate, o, options| {
            if let Some(predicate_iri) = get_iri_var("predicate", mstate)? {
                if o.data_property_declaration(&predicate_iri).is_some()
                    || options.is_data_prop(&predicate_iri)
                {
                    if let Some(subject) = mstate.get("subject") {
                        match subject {
                            Value::Iri(subject_iri) => {
                                return push_dataprop_assertion(
                                    subject_iri,
                                    // value,
                                    predicate_iri,
                                    mstate,
                                    o,
                                );
                            }
                            Value::Blank(_subject_bn) => {}
                            Value::Literal { .. } => unreachable!(),
                        }
                    }
                }
            }

            Ok(false)
        }),
    ));
    Ok(())
}

/// dataprop assertions
pub(crate) fn match_dataprop_assertions<'a>(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler<'a>)>,
    _prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("DataPropertyAssertion", _prefixes, 
            [iob:subject] [*:predicate] [lt:object] .)?,
        Box::new(|mstate, o, options| {
            if let Some(obj) = mstate.get("object") {
                let value: Literal = match obj.clone().try_into() {
                    Ok(l) => l,
                    Err(_) => unreachable!(),
                };
                if let Some(predicate_iri) = get_iri_var("predicate", mstate)? {
                    if o.data_property_declaration(&predicate_iri).is_some()
                        || options.is_data_prop(&predicate_iri)
                    {
                        if let Some(subject) = mstate.get("subject") {
                            match subject {
                                Value::Iri(_subject_iri) => {}
                                Value::Blank(subject_bn) => {
                                    return handle_dataprop_on_bn(
                                        o,
                                        subject_bn.clone(),
                                        predicate_iri,
                                        value,
                                    );
                                }
                                Value::Literal { .. } => unreachable!(),
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

pub(crate) fn handle_dataprop_on_bn(
    o: &mut OntologyCollector,
    subject_bn: harriet::triple_production::RdfBlankNode,
    predicate_iri: IRI,
    value: Literal,
) -> Result<bool, Error> {
    let annotate = o
        .annotation(CollectedReificationKey::Bn(subject_bn))
        .cloned();
    if annotate.is_none() {
        return Ok(false);
    }
    let annotate = annotate.unwrap();
    let subject = annotate.subject;
    let predicate = annotate.predicate;
    let object = annotate.object;

    if let Some((axiom, _)) = o.get_from_index_mut(&subject, &predicate, &object) {
        axiom
            .annotations_mut()
            .push(Annotation::new(predicate_iri.into(), value.into(), vec![]))
    }

    Ok(false)
}

fn push_dataprop_assertion(
    subject_iri: &str,
    predicate_iri: IRI,
    mstate: &MatcherState,
    o: &mut OntologyCollector,
) -> Result<bool, Error> {
    let subject_iri = IRI::new(subject_iri)?;

    if o.class_declaration(&subject_iri).is_some() {
        return Ok(false);
    }

    if let Some(object) = mstate.get("object") {
        match object {
            Value::Iri(_) => {
                return Ok(true);
            }
            Value::Literal { .. } => {
                if let Ok(lit) = object.clone().try_into() {
                    o.push_axiom(
                        DataPropertyAssertion::new(
                            predicate_iri.into(),
                            subject_iri.into(),
                            lit,
                            vec![],
                        )
                        .into(),
                    );
                    return Ok(true);
                }
                return Ok(false);
            }
            Value::Blank(_) => todo!(),
        }
    }
    Ok(false)
}
