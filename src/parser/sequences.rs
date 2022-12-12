use crate::parser::matcher::Value;

use crate::rdf_match;

use std::collections::HashMap;

use crate::error::Error;

use super::collector::{CollectedBlankNode, MatcherHandler};

use crate::parser::matcher::RdfMatcher;

pub(crate) fn match_sequences(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("sequences_first", prefixes,
            [:x] [rdf:first] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(Value::Blank(bn)) = mstate.get("x") {
                if let Some(object) = mstate.get("object").cloned() {
                    if o.get_blank(bn).is_some() {
                        println!("update {:?}, {}", bn, object);
                        o.update_blank_node_sequence(bn, Some(object), None);
                    } else {
                        println!("insert {:?}, {}", bn, object);
                        o.insert_blank_node(
                            bn.clone(),
                            CollectedBlankNode::Sequence {
                                first: Some(object),
                                rest: None,
                            },
                        );
                    }
                }
            }
            Ok(false)
        }),
    ));
    matchers.push((
        rdf_match!("sequences_rest", prefixes,
            [:x] [rdf:rest] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(Value::Blank(bn)) = mstate.get("x") {
                if let Some(Value::Blank(rest)) = mstate.get("object").cloned() {
                    if o.get_blank(bn).is_some() {
                        println!("R update {:?}, {:?}", bn, rest);

                        o.update_blank_node_sequence(bn, None, Some(rest));
                    } else {
                        println!("R insert {:?}, {:?}", bn, rest);
                        o.insert_blank_node(
                            bn.clone(),
                            CollectedBlankNode::Sequence {
                                first: None,
                                rest: Some(rest),
                            },
                        );
                    }
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}
