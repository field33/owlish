use crate::parser::matcher::Value;

use crate::rdf_match;

use std::collections::HashMap;

use crate::error::Error;

use super::collector::MatcherHandler;

use crate::parser::matcher::RdfMatcher;

pub(crate) fn match_sequences(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("sequences_first", prefixes,
            [:x] [rdf:first] [:object] .
        )?,
        Box::new(|mstate, o| {
            if let Some(Value::Blank(bn)) = mstate.get("x") {
                if let Some(object) = mstate.get("object").cloned() {
                    if let Some(sequence) = o.get_sequence(bn) {
                        sequence.push(object)
                    } else {
                        o.set_sequence_root(bn, object);
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
        Box::new(|mstate, o| {
            if let Some(Value::Blank(bn)) = mstate.get("x") {
                if let Some(Value::Blank(rest)) = mstate.get("object").cloned() {
                    o.set_sequence_tree(bn, rest)?;
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}
