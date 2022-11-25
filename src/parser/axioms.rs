use super::collector::CollectedBlankNode;
use super::collector::MatcherHandler;
use crate::error::Error;
use crate::get_vars;
use crate::owl::AnnotationPropertyDomain;
use crate::owl::AnnotationPropertyRange;
use crate::owl::ClassAssertion;
use crate::owl::ClassConstructor;
use crate::owl::DataPropertyDomain;
use crate::owl::DataPropertyRange;
use crate::owl::ObjectPropertyDomain;
use crate::owl::ObjectPropertyRange;
use crate::owl::SubClassOf;
use crate::owl::IRI;
use crate::parser::matcher::RdfMatcher;
use crate::parser::matcher::Value;
use crate::rdf_match;
use std::collections::HashMap;
use std::ops::Deref;

pub(crate) fn match_axioms(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
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
                            ClassAssertion::new(cls.into(), individual_iri.into(), vec![]).into(),
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
                            if let Some(CollectedBlankNode::ClassConstructor(constr)) =
                                o.get_blank(bn)
                            {
                                o.push_axiom(
                                    SubClassOf::new(
                                        ClassConstructor::IRI(IRI::new(subject_iri_str)?.into())
                                            .into(),
                                        constr.clone(),
                                        vec![],
                                    )
                                    .into(),
                                );
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
        rdf_match!("PropertyDomain", prefixes,
            [iob:subject] [rdfs:domain] [iob:object] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => {
                        if let Ok(op_iri) = IRI::new(subject_iri_str) {
                            match vars.object {
                                Value::Iri(object_iri_str) => {
                                    if let Ok(class_iri) = IRI::new(object_iri_str) {
                                        if o.data_property_declaration(&op_iri).is_some()
                                            || options.is_data_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                DataPropertyDomain::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.object_property_declaration(&op_iri).is_some()
                                            || options.is_object_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                ObjectPropertyDomain::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.annotation_property(&op_iri).is_some()
                                            || options.is_annotation(&op_iri)
                                        {
                                            o.push_axiom(
                                                AnnotationPropertyDomain::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        }
                                    }
                                }
                                Value::Blank(bn) => {
                                    if let Some(CollectedBlankNode::ClassConstructor(cc)) =
                                        o.get_blank(bn)
                                    {
                                        if o.data_property_declaration(&op_iri).is_some()
                                            || options.is_data_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                DataPropertyDomain::new(
                                                    op_iri.into(),
                                                    cc.deref().clone(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.object_property_declaration(&op_iri).is_some()
                                            || options.is_object_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                ObjectPropertyDomain::new(
                                                    op_iri.into(),
                                                    cc.deref().clone(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        }
                                    }
                                }
                                Value::Literal { .. } => {
                                    // TODO
                                }
                            }
                        }
                    }
                    Value::Blank(_) => todo!(),
                    Value::Literal { .. } => todo!(),
                }
            }
            Ok(false)
        }),
    ));

    matchers.push((
        rdf_match!("PropertyRange", prefixes,
            [iob:subject] [rdfs:range] [iob:object] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => {
                        if let Ok(op_iri) = IRI::new(subject_iri_str) {
                            match vars.object {
                                Value::Iri(object_iri_str) => {
                                    if let Ok(class_iri) = IRI::new(object_iri_str) {
                                        if o.data_property_declaration(&op_iri).is_some()
                                            || options.is_data_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                DataPropertyRange::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.object_property_declaration(&op_iri).is_some()
                                            || options.is_object_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                ObjectPropertyRange::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.annotation_property(&op_iri).is_some()
                                            || options.is_annotation(&op_iri)
                                        {
                                            o.push_axiom(
                                                AnnotationPropertyRange::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        }
                                    }
                                }
                                Value::Blank(bn) => {
                                    println!("rdfs:range for {:?}", bn);
                                    if let Some(cbn) = o.get_blank(bn) {
                                        println!("cbn {:?}", cbn);
                                        if let CollectedBlankNode::ClassConstructor(cc) = cbn {
                                            if o.object_property_declaration(&op_iri).is_some()
                                                || options.is_object_prop(&op_iri)
                                            {
                                                o.push_axiom(
                                                    ObjectPropertyRange::new(
                                                        op_iri.into(),
                                                        cc.deref().clone(),
                                                        vec![],
                                                    )
                                                    .into(),
                                                );
                                            }
                                        }
                                    }
                                }
                                Value::Literal { .. } => {
                                    // TODO
                                }
                            }
                        }
                    }
                    Value::Blank(_) => todo!(),
                    Value::Literal { .. } => todo!(),
                }
            }
            Ok(false)
        }),
    ));

    Ok(())
}
