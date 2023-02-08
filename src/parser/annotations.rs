use std::{collections::HashMap, convert::TryInto};

use crate::{
    error::Error,
    owl::{well_known, Annotation, AnnotationAssertion, Literal, LiteralOrIRI, IRI},
    parser::matcher::{RdfMatcher, Value},
    rdf_match,
};

use super::{
    collector::{
        get_iri_var, CollectedAnnotation, CollectedAnnotationKey, MatcherHandler, OntologyCollector,
    },
    matcher::MatcherState,
};

const WELL_KNOWN_ANNOTATIONS: [&str; 2] = [
    well_known::rdfs_label_str,
    well_known::rdfs_comment_str,
    //
];

/// annotations
/// https://www.w3.org/TR/2012/REC-owl2-mapping-to-rdf-20121211/#Parsing_of_Annotations
pub(crate) fn match_annotations<'a>(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler<'a>)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    // annotations on things
    matchers.push((
        rdf_match!("Annotation", prefixes,
            [iob:a] [rdf:type] [owl:Axiom] .
            [iob:a] [owl:annotatedSource] [:subject] .
            [iob:a] [owl:annotatedProperty] [*:predicate] .
            [iob:a] [owl:annotatedTarget] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(bn) = mstate.last("a") {
                match bn {
                    Value::Blank(bn) => {
                        if let Some(Value::Iri(subject)) = mstate.last("subject") {
                            if let Some(Value::Iri(predicate)) = mstate.last("predicate") {
                                match mstate.last("object") {
                                    Some(Value::Iri(object)) => o.insert_annotation(
                                        CollectedAnnotationKey::Bn(bn.clone()),
                                        CollectedAnnotation {
                                            subject: subject.clone(),
                                            predicate: predicate.clone(),
                                            object: object.clone(),
                                        },
                                    ),
                                    Some(Value::Literal {
                                        lexical_form,
                                        datatype_iri: _,
                                        language_tag: _,
                                    }) => {
                                        o.insert_annotation(
                                            CollectedAnnotationKey::Bn(bn.clone()),
                                            CollectedAnnotation {
                                                subject: subject.clone(),
                                                predicate: predicate.clone(),
                                                object: lexical_form.clone(),
                                            },
                                        );
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    }
                    Value::Iri(iri) => {
                        if let Some(Value::Iri(subject)) = mstate.last("subject") {
                            if let Some(Value::Iri(predicate)) = mstate.last("predicate") {
                                match mstate.last("object") {
                                    Some(Value::Iri(object)) => {
                                        o.insert_annotation(
                                            CollectedAnnotationKey::Iri(iri.clone()),
                                            CollectedAnnotation {
                                                subject: subject.clone(),
                                                predicate: predicate.clone(),
                                                object: object.clone(),
                                            },
                                        );
                                    }
                                    Some(Value::Literal {
                                        lexical_form,
                                        datatype_iri: _,
                                        language_tag: _,
                                    }) => {
                                        if let Some((axiom, index)) =
                                            o.get_from_index_mut(subject, predicate, lexical_form)
                                        {
                                            if let Ok(anno_iri) = IRI::new(iri) {
                                                axiom.annotations_mut().push(Annotation::new(
                                                    well_known::owl_annotatedSource().into(),
                                                    LiteralOrIRI::IRI(anno_iri),
                                                    vec![],
                                                ));
                                                o.insert_used_annotation(iri, index)
                                            }
                                        }
                                        o.insert_annotation(
                                            CollectedAnnotationKey::Iri(iri.clone()),
                                            CollectedAnnotation {
                                                subject: subject.clone(),
                                                predicate: predicate.clone(),
                                                object: lexical_form.clone(),
                                            },
                                        );
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    }
                    _ => todo!(),
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}

/// simple annotation assertions without blank nodes
/// https://www.w3.org/TR/2012/REC-owl2-mapping-to-rdf-20121211/#Parsing_of_Annotations
pub(crate) fn match_simple_annotation_assertions<'a>(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler<'a>)>,
    _prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("AnnotationAssertionSimple", _prefixes, 
            [iob:subject] [*:predicate] [iol:object] .)?,
        Box::new(|mstate, o, options| {
            if let Some(predicate_iri) = get_iri_var("predicate", mstate)? {
                if o.annotation_property_declaration(&predicate_iri).is_some()
                    || options.is_annotation_prop(&predicate_iri)
                    || WELL_KNOWN_ANNOTATIONS.contains(&predicate_iri.as_str())
                {
                    if let Some(subject) = mstate.get("subject") {
                        match subject {
                            Value::Iri(subject_iri) => {
                                return push_annotation_assertion(
                                    subject_iri,
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

/// annotation assertions
/// https://www.w3.org/TR/2012/REC-owl2-mapping-to-rdf-20121211/#Parsing_of_Annotations
pub(crate) fn match_annotation_assertions<'a>(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler<'a>)>,
    _prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("AnnotationAssertion", _prefixes, 
            [iob:subject] [*:predicate] [lt:object] .)?,
        Box::new(|mstate, o, options| {
            if let Some(obj) = mstate.get("object") {
                let value: Literal = match obj.clone().try_into() {
                    Ok(l) => l,
                    Err(_) => unreachable!(),
                };
                if let Some(predicate_iri) = get_iri_var("predicate", mstate)? {
                    if o.annotation_property_declaration(&predicate_iri).is_some()
                        || options.is_annotation_prop(&predicate_iri)
                        || WELL_KNOWN_ANNOTATIONS.contains(&predicate_iri.as_str())
                    {
                        if let Some(subject) = mstate.get("subject") {
                            match subject {
                                Value::Iri(subject_iri) => {
                                    // Here we handle AnnotationAssertions like <A> <p> <V> where <A> is an iri
                                    // that was previously defined via rdfs:annotatedSource as annotation on
                                    // another OWL axiom.
                                    // Those assertions need to be added to the annotation array of said OWL axiom.
                                    if let Some(annotations) =
                                        o.get_used_annotation(subject_iri).cloned()
                                    {
                                        for a in annotations {
                                            if let Some(axiom) = o.axiom_mut(a) {
                                                axiom.annotations_mut().push(Annotation::new(
                                                    predicate_iri.clone().into(),
                                                    value.clone().into(),
                                                    vec![],
                                                ))
                                            }
                                        }
                                    }
                                }
                                Value::Blank(subject_bn) => {
                                    return handle_annotation_on_bn(
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

fn handle_annotation_on_bn(
    o: &mut OntologyCollector,
    subject_bn: harriet::triple_production::RdfBlankNode,
    predicate_iri: IRI,
    value: Literal,
) -> Result<bool, Error> {
    let annotate = o
        .annotation(CollectedAnnotationKey::Bn(subject_bn))
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

fn push_annotation_assertion(
    subject_iri: &str,
    predicate_iri: IRI,
    mstate: &MatcherState,
    o: &mut OntologyCollector,
) -> Result<bool, Error> {
    if let Some(a) = o.annotation(CollectedAnnotationKey::Iri(subject_iri.into())) {
        println!("{:#?}", a);
    }
    let subject_iri = IRI::new(subject_iri)?;
    if let Some(object) = mstate.get("object") {
        match object {
            Value::Iri(object_iri) => {
                o.push_axiom(
                    AnnotationAssertion::new(
                        predicate_iri.into(),
                        subject_iri,
                        LiteralOrIRI::IRI(IRI::new(object_iri)?),
                        vec![],
                    )
                    .into(),
                );
                return Ok(true);
            }
            Value::Literal { .. } => {
                if let Ok(lit) = object.clone().try_into() {
                    o.push_axiom(
                        AnnotationAssertion::new(
                            predicate_iri.into(),
                            subject_iri,
                            LiteralOrIRI::Literal(lit),
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
