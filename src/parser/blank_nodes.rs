use crate::owl::ClassConstructor;

use crate::owl::IRI;

use crate::owl::ObjectIntersectionOf;
use crate::parser::matcher::Value;

use crate::rdf_match;

use crate::error::Error;

use std::collections::HashMap;

use crate::parser::matcher::RdfMatcher;

use super::collector::BlankNodeHandle;
use super::collector::MatcherHandler;

pub(crate) fn match_blank_nodes<'a>(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler<'a>)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    // TODO: parse all kinds of blank nodes to something like `Map<blank_node, AxiomOrDecl>`.

    // ObjectIntersectionOf
    matchers.push((
        rdf_match!("ObjectIntersectionOf", prefixes,
            [_:x] [rdf:type] [owl:Class] .
            [_:x] [owl:intersectionOf] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(Value::Blank(bn)) = mstate.get("x") {
                if let Some(Value::Blank(obj)) = mstate.get("object") {
                    if let Some(seq) = o.get_sequence(obj) {
                        if seq.iter().all(|v| matches!(v, Value::Iri(_))) {
                            let mut classes = Vec::new();
                            for v in seq {
                                if let Value::Iri(iri) = v {
                                    let iri = IRI::new(iri)?;
                                    classes.push(ClassConstructor::IRI(iri.into()));
                                }
                            }
                            o.insert_blank_node(
                                bn.clone(),
                                BlankNodeHandle::ClassConstructor(Box::new(
                                    ClassConstructor::ObjectIntersectionOf(ObjectIntersectionOf(
                                        classes,
                                        vec![],
                                    )),
                                )),
                            );
                            return Ok(true);
                        } else {
                            todo!("support deeper nested blank nodes")
                        }
                    } else {
                        return Err(Error::new("Could not find referenced sequence".into()));
                    }
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}