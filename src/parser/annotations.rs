use std::{collections::HashMap, convert::TryInto};

use crate::{
    error::Error,
    owl::{
        well_known, Annotation, AnnotationAssertion, Axiom, ClassConstructor, Literal,
        LiteralOrIRI, IRI,
    },
    parser::matcher::{RdfMatcher, Value},
    rdf_match,
};

use super::{
    collector::{get_iri_var, Ann, Annotate, MatcherHandler, OntologyCollector},
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
                                        Ann::Bn(bn.clone()),
                                        Annotate {
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
                                            Ann::Bn(bn.clone()),
                                            Annotate {
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
                                    Some(Value::Iri(object)) => o.insert_annotation(
                                        Ann::Iri(iri.clone()),
                                        Annotate {
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
                                            Ann::Iri(iri.clone()),
                                            Annotate {
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
                    if o.annotation_property(&predicate_iri).is_some()
                        || options.is_annotation(predicate_iri.as_str())
                        || WELL_KNOWN_ANNOTATIONS.contains(&predicate_iri.as_str())
                    {
                        if let Some(subject) = mstate.get("subject") {
                            match subject {
                                Value::Iri(subject_iri) => {
                                    if o.annotation(Ann::Iri(subject_iri.clone())).is_some() {
                                        return Ok(false);
                                    }
                                    if let Some(annotate) =
                                        o.annotation(Ann::Iri(subject_iri.clone())).cloned()
                                    {
                                        return handle_like_blank_but_with_iri(
                                            o,
                                            annotate,
                                            predicate_iri,
                                            value,
                                        );
                                    }

                                    return handle_annotation_on_iri(
                                        subject_iri,
                                        value,
                                        predicate_iri,
                                        mstate,
                                        o,
                                    );
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
    let annotate = o.annotation(Ann::Bn(subject_bn)).cloned();
    if annotate.is_none() {
        return Ok(false);
    }
    let annotate = annotate.unwrap();
    let subject = annotate.subject;
    let predicate = annotate.predicate;
    let object = annotate.object;

    for a in o.axioms_mut() {
        match a {
            Axiom::SubClassOf(sco) => {
                if sco.subject() == &ClassConstructor::IRI(IRI::new(&subject)?.into())
                    && predicate == well_known::rdfs_subClassOf().as_iri().as_str()
                    && sco.parent() == &ClassConstructor::IRI(IRI::new(&object)?.into())
                {
                    sco.2
                        .push(Annotation(predicate_iri.into(), value.into(), vec![]));
                    return Ok(true);
                }
            }
            Axiom::AnnotationAssertion(aa) => {
                if aa.subject() == &IRI::new(&subject)?
                    && aa.iri() == &IRI::new(&predicate)?.into()
                    && aa.value() == &Literal::String(object.to_string()).into()
                {
                    aa.3.push(Annotation(predicate_iri.into(), value.into(), vec![]));
                    return Ok(true);
                }
            }
            Axiom::SubObjectPropertyOf(_) => todo!(),
            Axiom::EquivalentObjectProperties(_) => {
                todo!()
            }
            Axiom::EquivalentDataProperties(_) => {
                todo!()
            }
            Axiom::InverseObjectProperties(_) => {
                todo!()
            }
            Axiom::DisjointObjectProperties(_) => {
                todo!()
            }
            Axiom::ObjectPropertyDomain(_) => todo!(),
            Axiom::ObjectPropertyRange(_) => todo!(),
            Axiom::DataPropertyDomain(_) => todo!(),
            Axiom::DataPropertyRange(_) => todo!(),
            Axiom::SymmetricObjectProperty(_) => {
                todo!()
            }
            Axiom::AsymmetricObjectProperty(_) => {
                todo!()
            }
            Axiom::ReflexiveObjectProperty(_) => {
                todo!()
            }
            Axiom::IrreflexiveObjectProperty(_) => {
                todo!()
            }
            Axiom::FunctionalObjectProperty(_) => {
                todo!()
            }
            Axiom::InverseFunctionalObjectProperty(_) => {
                todo!()
            }
            Axiom::TransitiveObjectProperty(_) => {
                todo!()
            }
            Axiom::FunctionalDataProperty(_) => todo!(),
            Axiom::EquivalentClasses(_) => todo!(),
            Axiom::DisjointClasses(_) => todo!(),
            Axiom::DatatypeDefinition(_) => todo!(),
            Axiom::ClassAssertion(_) => todo!(),
            Axiom::SameIndividual(_) => todo!(),
            Axiom::DifferentIndividuals(_) => todo!(),
            Axiom::ObjectPropertyAssertion(_) => {
                todo!()
            }
            Axiom::NegativeObjectPropertyAssertion(_) => {
                todo!()
            }
            Axiom::DataPropertyAssertion(_) => todo!(),
            Axiom::NegativeDataPropertyAssertion(_) => {
                todo!()
            }
            Axiom::HasKey(_) => todo!(),
        }
    }
    Ok(false)
}

fn handle_like_blank_but_with_iri(
    o: &mut OntologyCollector,
    annotate: Annotate,
    predicate_iri: IRI,
    value: Literal,
) -> Result<bool, Error> {
    let subject = annotate.subject;
    let predicate = annotate.predicate;
    let object = annotate.object;

    for a in o.axioms_mut() {
        match a {
            Axiom::SubClassOf(sco) => {
                if sco.subject() == &ClassConstructor::IRI(IRI::new(&subject)?.into())
                    && predicate == well_known::rdfs_subClassOf().as_iri().as_str()
                    && sco.parent() == &ClassConstructor::IRI(IRI::new(&object)?.into())
                {
                    sco.2
                        .push(Annotation(predicate_iri.into(), value.into(), vec![]));
                    return Ok(true);
                }
            }
            Axiom::AnnotationAssertion(aa) => {
                if aa.subject() == &IRI::new(&subject)?
                    && aa.iri() == &IRI::new(&predicate)?.into()
                    && aa.value() == &Literal::String(object.to_string()).into()
                {
                    aa.3.push(Annotation(predicate_iri.into(), value.into(), vec![]));
                    return Ok(true);
                }
            }
            Axiom::SubObjectPropertyOf(_) => todo!(),
            Axiom::EquivalentObjectProperties(_) => {
                todo!()
            }
            Axiom::EquivalentDataProperties(_) => {
                todo!()
            }
            Axiom::InverseObjectProperties(_) => {
                todo!()
            }
            Axiom::DisjointObjectProperties(_) => {
                todo!()
            }
            Axiom::ObjectPropertyDomain(_) => todo!(),
            Axiom::ObjectPropertyRange(_) => todo!(),
            Axiom::DataPropertyDomain(_) => todo!(),
            Axiom::DataPropertyRange(_) => todo!(),
            Axiom::SymmetricObjectProperty(_) => {
                todo!()
            }
            Axiom::AsymmetricObjectProperty(_) => {
                todo!()
            }
            Axiom::ReflexiveObjectProperty(_) => {
                todo!()
            }
            Axiom::IrreflexiveObjectProperty(_) => {
                todo!()
            }
            Axiom::FunctionalObjectProperty(_) => {
                todo!()
            }
            Axiom::InverseFunctionalObjectProperty(_) => {
                todo!()
            }
            Axiom::TransitiveObjectProperty(_) => {
                todo!()
            }
            Axiom::FunctionalDataProperty(_) => todo!(),
            Axiom::EquivalentClasses(_) => todo!(),
            Axiom::DisjointClasses(_) => todo!(),
            Axiom::DatatypeDefinition(_) => todo!(),
            Axiom::ClassAssertion(_) => todo!(),
            Axiom::SameIndividual(_) => todo!(),
            Axiom::DifferentIndividuals(_) => todo!(),
            Axiom::ObjectPropertyAssertion(_) => {
                todo!()
            }
            Axiom::NegativeObjectPropertyAssertion(_) => {
                todo!()
            }
            Axiom::DataPropertyAssertion(_) => todo!(),
            Axiom::NegativeDataPropertyAssertion(_) => {
                todo!()
            }
            Axiom::HasKey(_) => todo!(),
        }
    }
    Ok(false)
}

fn handle_annotation_on_iri(
    subject_iri: &str,
    _value: Literal,
    predicate_iri: IRI,
    mstate: &MatcherState,
    o: &mut OntologyCollector,
) -> Result<bool, Error> {
    let subject_iri = IRI::new(subject_iri)?;
    if let Some(object) = mstate.get("object") {
        match object {
            Value::Iri(object_iri) => {
                o.push_axiom(
                    AnnotationAssertion(
                        predicate_iri.into(),
                        subject_iri,
                        LiteralOrIRI::IRI(IRI::new(object_iri)?),
                        vec![],
                    )
                    .into(),
                );
                return Ok(true);
            }
            Value::Literal {
                lexical_form,
                datatype_iri,
                language_tag: _,
            } => {
                let lit: LiteralOrIRI = match datatype_iri {
                    Some(dt) if dt == well_known::xsd_integer_str => {
                        let lex = lexical_form;
                        LiteralOrIRI::Literal(Literal::Number {
                            number: serde_json::from_str(lex).map_err(|_| {
                                Error::new(format!(
                                    "Tried to parse a numeric value that was no number: {:?}",
                                    lex
                                ))
                            })?,
                            type_iri: Some(IRI::new(dt)?.into()),
                        })
                    }
                    _ => {
                        if let Ok(iri) = IRI::new(lexical_form) {
                            LiteralOrIRI::IRI(iri)
                        } else {
                            LiteralOrIRI::Literal(Literal::String(lexical_form.to_string()))
                        }
                    }
                };
                o.push_axiom(
                    AnnotationAssertion(predicate_iri.into(), subject_iri, lit, vec![]).into(),
                );
                return Ok(true);
            }
            Value::Blank(_) => todo!(),
        }
    }
    Ok(false)
}

fn _is_axiom<'a>(a: &'a mut Axiom, subject: &ClassConstructor) -> Option<&'a mut Vec<Annotation>> {
    match a {
        Axiom::SubClassOf(sco) => {
            let sub = sco.subject();
            if sub == subject {
                Some(&mut sco.2)
            } else {
                None
            }
        }
        Axiom::AnnotationAssertion(aa) => {
            let sub = aa.subject();
            if &ClassConstructor::IRI(sub.clone().into()) == subject {
                Some(&mut aa.3)
            } else {
                None
            }
        }
        _ => todo!(),
    }
}
