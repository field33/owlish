use harriet::triple_production::{RdfObject, RdfPredicate, RdfSubject, RdfTriple};
use serde::{Deserialize, Serialize};

use crate::owl::{Literal, LiteralOrIRI, IRI};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Triple {
    subject: IRI,
    predicate: IRI,
    value: LiteralOrIRI,
}

pub fn parse_triple(ttl: &str) -> Option<Triple> {
    let mut ttl_string = ttl.to_string();
    if !ttl.ends_with('.') {
        ttl_string = format!("{} .", ttl)
    }
    let ttl = harriet::TurtleDocument::parse_full(&ttl_string);
    if ttl.is_err() {
        eprintln!("{:?}", ttl.unwrap_err());
        return None;
    }
    let ttl = ttl.unwrap();

    if let Ok(triples) = harriet::triple_production::TripleProducer::produce_for_document(&ttl) {
        if let Some(t) = triples.into_iter().next() {
            return Triple::from_rdf(t);
        }
    }
    None
}

impl Triple {
    fn from_rdf(t: RdfTriple<'_>) -> Option<Self> {
        if let RdfSubject::IRI(sub) = t.subject {
            if let Ok(sub) = IRI::new(&sub.iri) {
                let RdfPredicate::IRI(pred) = t.predicate;

                if let Ok(pred) = IRI::new(&pred.iri) {
                    match t.object {
                        RdfObject::IRI(obj) => {
                            if let Ok(obj) = IRI::new(&obj.iri) {
                                return Some(Self {
                                    subject: sub,
                                    predicate: pred,
                                    value: obj.into(),
                                });
                            }
                        }
                        RdfObject::Literal(lit) => {
                            return Some(Self {
                                subject: sub,
                                predicate: pred,
                                value: Literal::from(lit.lexical_form.to_string()).into(),
                            });
                        }
                        RdfObject::BlankNode(_) => return None,
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::owl::IRI;

    use super::{parse_triple, Triple};

    #[test]
    fn tests() {
        let ttl = r#"<http://field33.com/a#1234> <http://field33.com/ontologies/business_object/Number> "1234""#;
        let t = parse_triple(ttl);
        assert_eq!(
            t,
            Some(Triple {
                subject: IRI::new("http://field33.com/a#1234").unwrap(),
                predicate: IRI::new("http://field33.com/ontologies/business_object/Number")
                    .unwrap(),
                value: "1234".into()
            })
        )
    }
}
