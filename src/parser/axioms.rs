use super::collector::CollectedBlankNode;
use super::collector::MatcherHandler;
use crate::error::Error;
use crate::get_vars;
use crate::owl::ClassAssertion;
use crate::owl::ClassConstructor;
use crate::owl::DataPropertyAssertion;
use crate::owl::ObjectPropertyDomain;
use crate::owl::ObjectPropertyRange;
use crate::owl::SubClassOf;
use crate::owl::IRI;
use crate::parser::matcher::RdfMatcher;
use crate::parser::matcher::Value;
use crate::rdf_match;
use std::collections::HashMap;

pub(crate) fn match_axioms(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("DataPropertyAssertion", prefixes,
            [+:x] [*:predicate] [lt:value] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(predicate) = mstate.last_iri("predicate") {
                let predicate_iri = IRI::new(predicate)?;
                if let Some(subject) = mstate.last("x") {
                    match subject {
                        Value::Iri(subject_iri) => {
                            let subject_iri = IRI::new(subject_iri)?;
                            if let Some(value) = mstate.last_literal("value") {
                                if o.data_property_declaration(&predicate_iri).is_some()
                                    || options.is_data_prop(predicate_iri.as_str())
                                {
                                    o.push_axiom(
                                        DataPropertyAssertion::new(
                                            predicate_iri.into(),
                                            subject_iri.into(),
                                            value,
                                            vec![],
                                        )
                                        .into(),
                                    )
                                }
                            }
                        }
                        Value::Blank(_) => {

                            // todo
                        }
                        Value::Literal { .. } => {
                            unreachable!("subject must not be a literal")
                        }
                    }
                }
            }

            //     }
            // }

            Ok(false)
        }),
    ));
    matchers.push((
        rdf_match!("ClassAssertions", prefixes,
            [*:x] [rdf:type] [*:cls] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(individual_iri) = mstate.last_iri("x") {
                if let Some(cls) = mstate.last_iri("cls") {
                    let individual_iri = IRI::new(individual_iri)?;
                    let cls = IRI::new(cls)?;
                    if o.class_declaration(&cls).is_some() {
                        o.push_axiom(
                            ClassAssertion::new(individual_iri.into(), cls.into(), vec![]).into(),
                        )
                    }
                }
            }

            Ok(false)
        }),
    ));
    matchers.push((
        rdf_match!("SubClassOf", prefixes,
            [:x] [rdfs:subClassOf] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(vars) = get_vars!(mstate, x, object) {
                match vars.x {
                    Value::Iri(subject_iri_str) => match vars.object {
                        Value::Iri(object_iri_str) => {
                            o.push_axiom(
                                SubClassOf::new(
                                    ClassConstructor::IRI(IRI::new(subject_iri_str)?.into()).into(),
                                    ClassConstructor::IRI(IRI::new(object_iri_str)?.into()).into(),
                                    vec![],
                                )
                                .into(),
                            );
                        }
                        Value::Blank(bn) => {
                            if let Some(bnh) = o.get_blank(bn) {
                                match bnh {
                                    CollectedBlankNode::ClassConstructor(constr) => {
                                        o.push_axiom(
                                            SubClassOf::new(
                                                ClassConstructor::IRI(
                                                    IRI::new(subject_iri_str)?.into(),
                                                )
                                                .into(),
                                                constr.clone(),
                                                vec![],
                                            )
                                            .into(),
                                        );
                                    }
                                }
                            }
                        }
                        Value::Literal { .. } => todo!(),
                    },
                    Value::Blank(_) => todo!(),
                    Value::Literal { .. } => todo!(),
                }
            }
            Ok(false)
        }),
    ));

    matchers.push((
        rdf_match!("ObjectPropertyDomain", prefixes,
            [iob:subject] [rdfs:domain] [iob:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => match vars.object {
                        Value::Iri(object_iri_str) => {
                            if let Ok(op_iri) = IRI::new(subject_iri_str) {
                                if let Ok(class_iri) = IRI::new(object_iri_str) {
                                    o.push_axiom(
                                        ObjectPropertyDomain::new(
                                            op_iri.into(),
                                            class_iri.into(),
                                            vec![],
                                        )
                                        .into(),
                                    );
                                }
                            }
                        }
                        Value::Blank(_) => {
                            todo!()
                        }
                        Value::Literal { .. } => todo!(),
                    },
                    Value::Blank(_) => todo!(),
                    Value::Literal { .. } => todo!(),
                }
            }
            Ok(false)
        }),
    ));

    matchers.push((
        rdf_match!("ObjectPropertyRange", prefixes,
            [iob:subject] [rdfs:range] [iob:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => match vars.object {
                        Value::Iri(object_iri_str) => {
                            if let Ok(op_iri) = IRI::new(subject_iri_str) {
                                if let Ok(class_iri) = IRI::new(object_iri_str) {
                                    o.push_axiom(
                                        ObjectPropertyRange::new(
                                            op_iri.into(),
                                            class_iri.into(),
                                            vec![],
                                        )
                                        .into(),
                                    );
                                }
                            }
                        }
                        Value::Blank(_) => {
                            todo!()
                        }
                        Value::Literal { .. } => todo!(),
                    },
                    Value::Blank(_) => todo!(),
                    Value::Literal { .. } => todo!(),
                }
            }
            Ok(false)
        }),
    ));

    Ok(())
}
