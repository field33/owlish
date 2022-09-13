use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    hash::Hash,
    rc::Rc,
};

use harriet::triple_production::{RdfBlankNode, RdfLiteral, RdfObject, RdfSubject, RdfTriple};
use log::debug;

use crate::{
    owl::{well_known, Literal, IRI},
    parser_debug,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Iri(String),
    Blank(RdfBlankNode),
    Literal {
        lexical_form: String,
        datatype_iri: Option<String>,
        language_tag: Option<String>,
    },
}

impl TryInto<Literal> for Value {
    type Error = ();

    fn try_into(self) -> Result<Literal, Self::Error> {
        match self {
            Value::Literal {
                lexical_form,
                datatype_iri,
                language_tag: _,
            } => {
                if let Some(s) = datatype_iri {
                    match s.as_str() {
                        well_known::xsd_string_str => Ok(Literal::String(lexical_form)),
                        well_known::xsd_integer_str => serde_json::from_str(&lexical_form)
                            .map_err(|_| ())
                            .map(|n| Literal::Number {
                                number: n,
                                type_iri: well_known::xsd_integer().into(),
                            }),
                        well_known::xsd_float_str => serde_json::from_str(&lexical_form)
                            .map_err(|_| ())
                            .map(|n| Literal::Number {
                                number: n,
                                type_iri: well_known::xsd_float().into(),
                            }),
                        iri => IRI::new(iri).map_err(|_| ()).map(|iri| Literal::Raw {
                            data: lexical_form.as_bytes().to_vec(),
                            type_iri: iri.into(),
                        }),
                    }
                } else {
                    Ok(Literal::String(lexical_form))
                }
            }
            Value::Iri(_) => Err(()),
            Value::Blank(_) => Err(()),
        }
    }
}

impl<'a> From<RdfLiteral<'a>> for Value {
    fn from(lit: RdfLiteral<'a>) -> Self {
        Value::Literal {
            lexical_form: lit.lexical_form.into(),
            datatype_iri: lit.datatype_iri.map(|d| d.iri.into()),
            language_tag: lit.language_tag.map(|t| t.into()),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Iri(iri) => write!(f, "{}", iri),
            Value::Blank(bn) => write!(f, "{:?}", bn),
            Value::Literal {
                lexical_form,
                datatype_iri: _,
                language_tag: _,
            } => write!(f, "{}", lexical_form),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchOrVar {
    Iri(String),
    Blank(RdfBlankNode),
    Var(&'static str),
    IriVar(&'static str),
    BlankVar(&'static str),
    LitVar(&'static str),
    IriOrBlankVar(&'static str),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum IRIOrBlank {
    Iri(String),
    Blank(RdfBlankNode),
}

impl From<IRIOrBlank> for Value {
    fn from(iob: IRIOrBlank) -> Self {
        match iob {
            IRIOrBlank::Iri(iri) => Value::Iri(iri),
            IRIOrBlank::Blank(b) => Value::Blank(b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatcherState {
    variables: HashMap<String, Vec<Value>>,
    matches: Vec<usize>,
}

impl MatcherState {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
            matches: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }
    pub fn push(&mut self, var: &str, value: Value) {
        if let Some(values) = self.variables.get_mut(var) {
            values.push(value);
        } else {
            self.variables.insert(var.into(), vec![value]);
        }
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.variables.get(key).and_then(|v| v.last())
    }
    pub fn last(&self, key: &str) -> Option<&Value> {
        self.variables.get(key).and_then(|v| v.last())
    }

    pub fn last_iri(&self, name: &str) -> Option<&String> {
        self.variables.get(name).and_then(|vs| {
            let mut value = None;
            for v in vs {
                if let Value::Iri(iri) = v {
                    value = Some(iri)
                }
            }
            value
        })
    }

    pub fn last_literal(&self, name: &str) -> Option<Literal> {
        self.variables.get(name).and_then(|vs| {
            let mut value = None;
            for v in vs {
                if let Value::Literal { .. } = v {
                    value = v.clone().try_into().ok()
                }
            }
            value
        })
    }

    pub(crate) fn matched(&mut self, matcher_id: usize) {
        if let Some(last) = self.matches.last() {
            if *last != matcher_id {
                self.matches.push(matcher_id);
            }
        } else {
            self.matches.push(matcher_id);
        }
    }

    pub(crate) fn check_var(&self, var: &str, value: Value) -> (bool, Option<(String, Value)>) {
        if let Some(variable) = self.variables.get(var).and_then(|v| v.last()) {
            (variable == &value, None)
        } else {
            let mut value_in_other_variable = false;
            for (existing_var, values) in &self.variables {
                if let Some(v) = values.last() {
                    if var != existing_var && v == &value {
                        value_in_other_variable = true;
                    }
                }
            }
            if value_in_other_variable {
                (false, None)
            } else {
                (true, Some((var.to_string(), value)))
            }
        }
    }
}

impl<'a> From<RdfSubject<'a>> for MatchOrVar {
    fn from(s: RdfSubject<'a>) -> Self {
        match s {
            RdfSubject::IRI(iri) => MatchOrVar::Iri(iri.iri.into()),
            RdfSubject::BlankNode(b) => MatchOrVar::Blank(b),
        }
    }
}

impl<'a> From<RdfSubject<'a>> for IRIOrBlank {
    fn from(s: RdfSubject<'a>) -> Self {
        match s {
            RdfSubject::IRI(iri) => IRIOrBlank::Iri(iri.iri.into()),
            RdfSubject::BlankNode(b) => IRIOrBlank::Blank(b),
        }
    }
}

pub enum MatchResult {
    Matched(bool),
    Nope,
}

// impl std::fmt::Display for RdfMatcher {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "Matcher {} {{", self.name)?;
//         for t in &self.match_triples {
//             writeln!(f, "  [{}] [{}] [{}] .", t.0, t.1, t.2)?;
//         }
//         write!(f, "}}")
//     }
// }

impl std::fmt::Display for MatchOrVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchOrVar::Iri(iri) => write!(f, "{}", iri),
            MatchOrVar::Blank(bn) => {
                write!(f, "{:?}", bn)
            }
            MatchOrVar::Var(var) => write!(f, ":{}", var),
            MatchOrVar::BlankVar(var) => write!(f, "_:{}", var),
            MatchOrVar::IriVar(var) => write!(f, "*:{}", var),
            MatchOrVar::LitVar(var) => write!(f, "lt:{}", var),
            MatchOrVar::IriOrBlankVar(var) => write!(f, "iob:{}", var),
        }
    }
}

pub fn print(matcher: &RdfMatcher, mstate: &MatcherState) -> String {
    let mut s = format!("Matcher {} {{\n", matcher.name);
    for (i, t) in matcher.match_triples.iter().enumerate() {
        if let Some(l) = mstate.matches.last() {
            if l + 1 == i {
                s = format!("{}> [{}] [{}] [{}] .\n", s, t.0, t.1, t.2);
            } else {
                s = format!("{}  [{}] [{}] [{}] .\n", s, t.0, t.1, t.2);
            }
        } else if i == 0 {
            s = format!("{}> [{}] [{}] [{}] .\n", s, t.0, t.1, t.2);
        } else {
            s = format!("{}  [{}] [{}] [{}] .\n", s, t.0, t.1, t.2);
        }
    }
    format!("{} }}", s)
}

pub fn display(triple: &RdfTriple) -> String {
    let sub = match &triple.subject {
        RdfSubject::IRI(iri) => format!("{}", iri.iri),
        RdfSubject::BlankNode(bn) => {
            format!("{:?}", bn)
        }
    };
    let pre = match &triple.predicate {
        harriet::triple_production::RdfPredicate::IRI(iri) => format!("{}", iri.iri),
    };
    let obj = match &triple.object {
        RdfObject::IRI(iri) => format!("{}", iri.iri),
        RdfObject::BlankNode(bn) => {
            format!("{:?}", bn)
        }
        RdfObject::Literal(lit) => {
            format!("'{}'", lit.lexical_form)
        }
    };
    format!("[{} {} {} .]", sub, pre, obj)
}

#[derive(Debug, PartialEq, Eq)]
pub struct RdfMatcher {
    name: String,
    match_triples: Vec<(MatchOrVar, MatchOrVar, MatchOrVar)>,
}

impl RdfMatcher {
    pub fn new(name: &str, match_triples: Vec<(MatchOrVar, MatchOrVar, MatchOrVar)>) -> Self {
        Self {
            name: name.into(),
            match_triples,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn matches<'a>(&self, triple: Rc<RdfTriple<'a>>, mstate: &mut MatcherState) -> MatchResult {
        let triple_matches = self.match_triple(triple, mstate);

        let mut expected_variables = HashSet::new();
        for triple in &self.match_triples {
            let (s, p, o) = triple;
            match s {
                MatchOrVar::Var(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::IriVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::LitVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::BlankVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::IriOrBlankVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::Iri(_) => {}
                MatchOrVar::Blank(_) => {}
            }
            match p {
                MatchOrVar::Var(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::IriVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::LitVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::BlankVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::IriOrBlankVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::Iri(_) => {}
                MatchOrVar::Blank(_) => {}
            }
            match o {
                MatchOrVar::Var(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::IriVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::LitVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::BlankVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::IriOrBlankVar(var) => {
                    expected_variables.insert(var.to_string());
                }
                MatchOrVar::Iri(_) => {}
                MatchOrVar::Blank(_) => {}
            }
        }

        let finished = expected_variables.len() == mstate.len()
            && mstate.matches.len() == self.match_triples.len();

        parser_debug!(self, "{:?}", finished);

        if triple_matches {
            if finished {
                MatchResult::Matched(true)
            } else {
                MatchResult::Matched(false)
            }
        } else {
            MatchResult::Nope
        }
    }

    fn match_triple<'a>(&self, triple: Rc<RdfTriple<'a>>, mstate: &mut MatcherState) -> bool {
        parser_debug!(
            self,
            "#############################################################"
        );
        parser_debug!(self, "{}", display(&triple));
        parser_debug!(self, "{}", print(self, mstate));

        let mut triple_matches = false;

        for (matcher_id, (subject_matcher, predicate_matcher, object_matcher)) in
            self.match_triples.iter().enumerate()
        {
            let (subject_matches, subject_variable) = match &triple.subject {
                RdfSubject::IRI(iri) => match subject_matcher {
                    MatchOrVar::Iri(p_iri) => (*p_iri == iri.iri, None),
                    MatchOrVar::Blank(_) => (false, None),
                    MatchOrVar::Var(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriOrBlankVar(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriVar(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::BlankVar(_) => (false, None),
                    MatchOrVar::LitVar(_) => (false, None),
                },
                RdfSubject::BlankNode(bn) => match subject_matcher {
                    MatchOrVar::Iri(_) => (false, None),
                    MatchOrVar::Blank(p_bn) => (bn == p_bn, None),
                    MatchOrVar::LitVar(_) => (false, None),
                    MatchOrVar::Var(var) => {
                        let value = Value::Blank(bn.clone());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriOrBlankVar(var) => {
                        let value = Value::Blank(bn.clone());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriVar(_) => (false, None),
                    MatchOrVar::BlankVar(var) => {
                        let value = Value::Blank(bn.clone());
                        mstate.check_var(*var, value)
                    }
                },
            };

            // Check whether the predicate matches or update variables
            let (predicate_matches, predicate_variable) = match &triple.predicate {
                harriet::triple_production::RdfPredicate::IRI(iri) => match predicate_matcher {
                    MatchOrVar::Iri(p_iri) => (*p_iri == iri.iri, None),
                    MatchOrVar::Blank(_) => (false, None),
                    MatchOrVar::Var(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriOrBlankVar(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriVar(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::BlankVar(_) => (false, None),
                    MatchOrVar::LitVar(_) => (false, None),
                },
            };

            // Check whether the object matches or update variables
            let (object_matches, object_variable) = match &triple.object {
                RdfObject::IRI(iri) => match object_matcher {
                    MatchOrVar::Iri(p_iri) => (*p_iri == iri.iri, None),
                    MatchOrVar::Blank(_) => (false, None),
                    MatchOrVar::Var(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriOrBlankVar(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriVar(var) => {
                        let value = Value::Iri(iri.iri.to_string());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::BlankVar(_) => (false, None),
                    MatchOrVar::LitVar(_) => (false, None),
                },
                RdfObject::BlankNode(bn) => match object_matcher {
                    MatchOrVar::Iri(_) => (false, None),
                    MatchOrVar::Blank(p_bn) => (bn == p_bn, None),
                    MatchOrVar::LitVar(_) => (false, None),
                    MatchOrVar::Var(var) => {
                        let value = Value::Blank(bn.clone());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriOrBlankVar(var) => {
                        let value = Value::Blank(bn.clone());
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriVar(_) => (false, None),
                    MatchOrVar::BlankVar(var) => {
                        let value = Value::Blank(bn.clone());
                        mstate.check_var(*var, value)
                    }
                },
                RdfObject::Literal(lit) => match object_matcher {
                    MatchOrVar::Iri(_) => (false, None),
                    MatchOrVar::Blank(_) => (false, None),
                    MatchOrVar::Var(var) => {
                        let value = lit.clone().into();
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriVar(_) => (false, None),
                    MatchOrVar::BlankVar(_) => (false, None),
                    MatchOrVar::LitVar(var) => {
                        let value = lit.clone().into();
                        mstate.check_var(*var, value)
                    }
                    MatchOrVar::IriOrBlankVar(_) => (false, None),
                },
            };

            if subject_matches && predicate_matches && object_matches {
                parser_debug!(self, "Matched: #{}", matcher_id);
                if let Some(v) = subject_variable {
                    mstate.push(&v.0, v.1);
                }
                if let Some(v) = predicate_variable {
                    mstate.push(&v.0, v.1);
                }
                if let Some(v) = object_variable {
                    mstate.push(&v.0, v.1);
                }
                triple_matches = true;
                mstate.matched(matcher_id);
                // parser_debug!(self, "{:?}", mstate);
            } else {
                parser_debug!(self, "{} No match. Next...", matcher_id);
            }
        }
        triple_matches
    }
}

/// Creates a matcher from the given input. Requires a human readable name the map of used prefixes:
///
/// # Example:
/// ```ignore
/// rdf_match!("Test", prefixes,
///     [:x] [rdf:type] [owl:Class] .
/// );
/// ```
///
/// The syntax of the triple matchers is based on this document:
/// https://www.w3.org/TR/2012/REC-owl2-mapping-to-rdf-20121211/#Parsing_of_Annotations
///
/// ```ignore
/// [*:x] [B] [C] .
/// ```
///
/// will parse the subject of the triple as an IRI.
///
/// ```ignore
/// [iob:x] [B] [C] .
/// [+:y] [B] [C] .
/// ```
///
/// will parse x and y as IRI or blank nodes.
///
/// ```ignore
/// [_:x] [A] [B] .
/// ```
///
/// will parse x as a blank node.
///
/// ```ignore
/// [lt:x] [A] [B] .
/// ```
///
/// will parse x as a litral.
///
///
/// You can define multiple matchers in one like this:
///
/// ```ignore
/// [:subject] [*:predicate] [:object] .
/// [_:a] [rdf:type] [owl:Axiom] .
/// [_:a] [owl:annotatedSource] [:subject] .
/// [_:a] [owl:annotatedProperty] [*:predicate] .
/// [_:a] [owl:annotatedTarget] [:object] .
/// [_:a] [*:annotation] [lt:value] .
/// ```
///
/// Which means the matcher will only apply when a list
/// of triples contains 6 triples that match the rules.
///
/// It also means that the 4. rule matches only a triple
/// that has a predicate variable that is the same as the predicate variable in the first rule.
///
#[macro_export]
macro_rules! rdf_match {
    ( $name:literal, $prefixes:ident, $(
        [$($subject:tt)+] [$($predicate:tt)+] [$($object:tt)+] .
    )+ ) => {{
        let mut match_triples: Vec<Result<(
            $crate::parser::matcher::MatchOrVar,
            $crate::parser::matcher::MatchOrVar,
            $crate::parser::matcher::MatchOrVar
        ), String>> = Vec::new();
        $(
            match_triples.push($crate::triple_match!($prefixes, [$($subject)+] [$($predicate)+] [$($object)+] .));
        )+
        let errors = match_triples.iter().filter_map(|e| e.clone().err()).collect::<Vec<String>>();
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok($crate::parser::matcher::RdfMatcher::new(
                $name,
                match_triples.into_iter().filter_map(|p| p.ok()).collect(),
            ))
        }
    }};
}

#[macro_export]
macro_rules! matcher_or_var {
    (
        $prefixes:ident,
        :$($var:ident)+
    ) => {{
        let var = stringify!($($var)+);
        Ok($crate::parser::matcher::MatchOrVar::Var(var))
    }};

    (
        $prefixes:ident,
        *:$($var:ident)+
    ) => {{
        let var = stringify!($($var)+);
        Ok($crate::parser::matcher::MatchOrVar::IriVar(var))
    }};

    (
        $prefixes:ident,
        iob:$($var:ident)+
    ) => {{
        let var = stringify!($($var)+);
        Ok($crate::parser::matcher::MatchOrVar::IriOrBlankVar(var))
    }};
    (
        $prefixes:ident,
        +:$($var:ident)+
    ) => {{
        let var = stringify!($($var)+);
        Ok($crate::parser::matcher::MatchOrVar::IriOrBlankVar(var))
    }};


    (
        $prefixes:ident,
        lt:$($var:ident)+
    ) => {{
        let var = stringify!($($var)+);
        Ok($crate::parser::matcher::MatchOrVar::LitVar(var))
    }};

    (
        $prefixes:ident,
        _:$($var:ident)+
    ) => {{
        let var = stringify!($($var)+);
        Ok($crate::parser::matcher::MatchOrVar::BlankVar(var))
    }};

    (
        $prefixes:ident,
        $($pre:ident)+:$($suf:ident)+
    ) => {{
        let prefix = stringify!($($pre)+);
        let suffix = stringify!($($suf)+);
        if let Some(prefix) = $prefixes.get(prefix) {
            Ok(
                $crate::parser::matcher::MatchOrVar::Iri(format!("{}{}", prefix, suffix)),
            )
        } else {
            Err(format!("Unkonwn prefix: '{}'", prefix))
        }

    }};
}

#[macro_export]
macro_rules! triple_match {
    (
        $prefixes:ident,
        [$($subject:tt)+] [$($predicate:tt)+] [$($object:tt)+] .
    ) => {{
        let sub: Result<$crate::parser::matcher::MatchOrVar, String> = $crate::matcher_or_var!($prefixes, $($subject)+);
        let pre: Result<$crate::parser::matcher::MatchOrVar, String> = $crate::matcher_or_var!($prefixes, $($predicate)+);
        let obj: Result<$crate::parser::matcher::MatchOrVar, String> = $crate::matcher_or_var!($prefixes, $($object)+);
        if sub.is_err() {
            Err(sub.unwrap_err())
        } else if pre.is_err() {
            Err(pre.unwrap_err())
        } else if obj.is_err() {
            Err(obj.unwrap_err())
        } else {
            Ok((
                #[allow(clippy::unnecessary_unwrap)]
                sub.unwrap(),
                #[allow(clippy::unnecessary_unwrap)]
                pre.unwrap(),
                #[allow(clippy::unnecessary_unwrap)]
                obj.unwrap()
            ))
        }
    }};
}

pub fn get_prefixes(ttl: harriet::TurtleDocument) -> HashMap<String, String> {
    let mut prefixes: HashMap<String, String> = HashMap::new();

    for s in ttl.statements {
        match s {
            harriet::Statement::Directive(d) => {
                if let harriet::Directive::Prefix(prefix) = d {
                    if let Some(p) = prefix.prefix {
                        prefixes.insert(p.to_string(), prefix.iri.iri.to_string());
                    }
                }
            }
            harriet::Statement::Triples(_) => break,
        }
    }
    prefixes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_object_has_value() {
        let ttl = r#"
            @prefix : <http://test#> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix xml: <http://www.w3.org/XML/1998/namespace> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            <http://test#A/> rdf:type owl:Ontology .

            :Parent  owl:equivalentClass  [
                rdf:type            owl:Restriction ;
                owl:onProperty      :hasChild ;
                owl:hasValue        :Person
            ] .
        "#;

        let ttl = harriet::TurtleDocument::parse_full(ttl).unwrap();
        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .unwrap()
                .into_iter()
                .map(Rc::new)
                .collect();

        let mut variables = MatcherState::new();
        let m = RdfMatcher::new(
            "test",
            vec![
                (
                    crate::parser::matcher::MatchOrVar::Var("subject"),
                    crate::parser::matcher::MatchOrVar::Iri(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".into(),
                    ),
                    MatchOrVar::Iri("http://www.w3.org/2002/07/owl#Restriction".into()),
                ),
                (
                    crate::parser::matcher::MatchOrVar::Var("subject"),
                    crate::parser::matcher::MatchOrVar::Iri(
                        "http://www.w3.org/2002/07/owl#onProperty".into(),
                    ),
                    MatchOrVar::Var("predicate"),
                ),
                (
                    crate::parser::matcher::MatchOrVar::Var("subject"),
                    crate::parser::matcher::MatchOrVar::Iri(
                        "http://www.w3.org/2002/07/owl#hasValue".into(),
                    ),
                    MatchOrVar::Var("object"),
                ),
            ],
        );

        let mut matches = 0;
        for triple in triples.iter() {
            if let MatchResult::Matched(true) = &m.matches(triple.clone(), &mut variables) {
                matches += 1;
            }
        }
        assert_eq!(matches, 1);
        assert_eq!(
            "ObjectHasValue(http://test#hasChild, http://test#Person)",
            format!(
                "ObjectHasValue({}, {})",
                variables.get("predicate").unwrap(),
                variables.get("object").unwrap()
            )
        );
    }

    #[test]
    fn iri() {
        env_logger::try_init().ok();
        let ttl = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :test1 rdf:type owl:DatatypeProperty .
        :test2 rdf:type owl:DatatypeProperty .
        :test3 rdf:type owl:DatatypeProperty .
        :test4 rdf:type owl:DatatypeProperty .
        :test5 rdf:type owl:DatatypeProperty .

        "##;

        let ttl = harriet::TurtleDocument::parse_full(ttl).unwrap();
        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .unwrap()
                .into_iter()
                .map(Rc::new)
                .collect();

        let prefixes = get_prefixes(ttl);
        let mut variables = MatcherState::new();
        let m = rdf_match!("test", prefixes,
            [*:x] [rdf:type] [owl:Ontology] .
        )
        .unwrap();

        let mut matches = 0;
        for triple in triples.iter() {
            if let MatchResult::Matched(true) = m.matches(triple.clone(), &mut variables) {
                matches += 1;
            }
        }
        assert_eq!(matches, 1);
    }

    #[test]
    fn literal() {
        env_logger::try_init().ok();
        let ttl = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :test1 rdfs:label "Test" .

        "##;

        let ttl = harriet::TurtleDocument::parse_full(ttl).unwrap();
        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .unwrap()
                .into_iter()
                .map(Rc::new)
                .collect();

        let prefixes = get_prefixes(ttl);
        let mut variables = MatcherState::new();
        let m = rdf_match!("test", prefixes,
            [*:x] [rdfs:label] [lt:value] .
        )
        .unwrap();

        let mut matches = 0;
        for triple in triples.iter() {
            if let MatchResult::Matched(true) = m.matches(triple.clone(), &mut variables) {
                matches += 1;
            }
        }
        assert_eq!(matches, 1);
    }

    #[test]
    fn test_match_object_has_value_with_macro() {
        env_logger::try_init().ok();
        let ttl = r#"
            @prefix : <http://test#> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix xml: <http://www.w3.org/XML/1998/namespace> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            <http://test#A/> rdf:type owl:Ontology .

            :Parent  owl:equivalentClass  [
                rdf:type            owl:Restriction ;
                owl:onProperty      :hasChild ;
                owl:hasValue        :Person
            ] .
        "#;

        let ttl = harriet::TurtleDocument::parse_full(ttl).unwrap();
        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .unwrap()
                .into_iter()
                .map(Rc::new)
                .collect();

        let prefixes = get_prefixes(ttl);
        let mut mstate = MatcherState::new();
        let m = rdf_match!("test", prefixes,
            [_:x] [rdf:type] [owl:Restriction] .
            [_:x] [owl:onProperty] [:predicate] .
            [_:x] [owl:hasValue] [:object] .
        )
        .unwrap();

        let mut matches = 0;
        for triple in triples.iter() {
            if let MatchResult::Matched(true) = m.matches(triple.clone(), &mut mstate) {
                matches += 1;
            }
        }
        assert_eq!(matches, 1);
        assert_eq!(
            "ObjectHasValue(http://test#hasChild, http://test#Person)",
            format!(
                "ObjectHasValue({}, {})",
                mstate.get("predicate").unwrap(),
                mstate.get("object").unwrap()
            )
        );
    }

    #[test]
    fn match_intersection_of() {
        env_logger::try_init().ok();
        let ttl = r#"
            @prefix : <http://test#> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix xml: <http://www.w3.org/XML/1998/namespace> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            <http://test#A/> rdf:type owl:Ontology .

            :A rdf:type owl:Class .
            :A rdf:first :C .
            :A owl:intersectionOf :B .
        "#;

        let ttl = harriet::TurtleDocument::parse_full(ttl).unwrap();
        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .unwrap()
                .into_iter()
                .map(Rc::new)
                .collect();

        let prefixes = get_prefixes(ttl);
        let mut variables = MatcherState::new();
        let m = rdf_match!("test", prefixes,
            [*:x] [rdf:type] [owl:Class] .
            [*:x] [owl:intersectionOf] [*:object] .
        )
        .unwrap();

        let mut matches = 0;
        for triple in triples.iter() {
            if let MatchResult::Matched(true) = m.matches(triple.clone(), &mut variables) {
                matches += 1;
            }
        }
        assert_eq!(matches, 1);
        assert_eq!(
            "http://test#A intersect http://test#B",
            format!(
                "{} intersect {}",
                variables.get("x").unwrap(),
                variables.get("object").unwrap()
            )
        );
    }

    #[test]
    fn match_intersection_of_matches_not_too_much() {
        let ttl = r#"
            @prefix : <http://test#> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix xml: <http://www.w3.org/XML/1998/namespace> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            <http://test#A/> rdf:type owl:Ontology .
            
            :Woman rdf:type owl:Class .
            :Mother rdf:type owl:Class .
            :Parent rdf:type owl:Class .    

            :Woman   rdfs:subClassOf :Person .
            :Mother  rdfs:subClassOf :Woman .

            :Grandfather  rdfs:subClassOf  [
                rdf:type            owl:Class ;
                owl:intersectionOf  ( :Man  :Parent )
            ] .
        "#;

        let ttl = harriet::TurtleDocument::parse_full(ttl).unwrap();
        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .unwrap()
                .into_iter()
                .map(Rc::new)
                .collect();

        let prefixes = get_prefixes(ttl);
        let mut variables = MatcherState::new();
        let m = rdf_match!("test", prefixes,
            [_:x] [rdf:type] [owl:Class] .
            [_:x] [owl:intersectionOf] [_:object] .
        )
        .unwrap();

        let mut matches = 0;
        for triple in triples.iter() {
            if let MatchResult::Matched(true) = m.matches(triple.clone(), &mut variables) {
                matches += 1;
            }
        }
        assert_eq!(matches, 1);
    }

    #[test]
    fn value_parsing() {
        let value: Value = Value::Literal {
            lexical_form: "0.9".into(),
            datatype_iri: Some(well_known::xsd_float_str.into()),
            language_tag: None,
        };

        let literal: Literal = value.try_into().unwrap();

        assert_eq!(
            literal,
            Literal::Number {
                number: serde_json::from_str("0.9").unwrap(),
                type_iri: well_known::xsd_float().into()
            }
        )
    }
}
