use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;

use crate::owl::IRI;

use super::{well_known, DatatypeIRI, Lang};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum LiteralOrIRI {
    IRI(IRI),
    Literal(Literal),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Literal {
    Raw {
        data: Vec<u8>,
        type_iri: DatatypeIRI,
    },
    String(String),
    DateTime(String),
    LangString {
        string: String,
        lang: Lang,
    },
    Number {
        number: serde_json::Number,
        type_iri: Option<DatatypeIRI>,
    },
    Bool(bool),
}

impl From<IRI> for LiteralOrIRI {
    fn from(iri: IRI) -> Self {
        Self::IRI(iri)
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}
impl From<String> for Literal {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for LiteralOrIRI {
    fn from(s: &str) -> Self {
        Self::Literal(Literal::from(s))
    }
}
impl From<String> for LiteralOrIRI {
    fn from(s: String) -> Self {
        Self::Literal(Literal::from(s))
    }
}

impl From<u8> for Literal {
    fn from(u: u8) -> Self {
        Self::Number {
            number: u.into(),
            type_iri: well_known::xsd_nonNegativeInteger().into(),
        }
    }
}
impl From<u16> for Literal {
    fn from(u: u16) -> Self {
        Self::Number {
            number: u.into(),
            type_iri: well_known::xsd_nonNegativeInteger().into(),
        }
    }
}
impl From<u32> for Literal {
    fn from(u: u32) -> Self {
        Self::Number {
            number: u.into(),
            type_iri: well_known::xsd_nonNegativeInteger().into(),
        }
    }
}
impl From<u64> for Literal {
    fn from(u: u64) -> Self {
        Self::Number {
            number: u.into(),
            type_iri: well_known::xsd_nonNegativeInteger().into(),
        }
    }
}

impl From<i8> for Literal {
    fn from(i: i8) -> Self {
        Self::Number {
            number: i.into(),
            type_iri: well_known::xsd_integer().into(),
        }
    }
}
impl From<i16> for Literal {
    fn from(i: i16) -> Self {
        Self::Number {
            number: i.into(),
            type_iri: well_known::xsd_integer().into(),
        }
    }
}
impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Self::Number {
            number: i.into(),
            type_iri: well_known::xsd_integer().into(),
        }
    }
}
impl From<i64> for Literal {
    fn from(i: i64) -> Self {
        Self::Number {
            number: i.into(),
            type_iri: well_known::xsd_integer().into(),
        }
    }
}

impl TryFrom<f64> for Literal {
    type Error = ();

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        match serde_json::Number::from_f64(f) {
            Some(f) => Ok(Self::Number {
                number: f,
                type_iri: well_known::xsd_float().into(),
            }),
            None => Err(()),
        }
    }
}

impl From<(serde_json::Number, Option<DatatypeIRI>)> for Literal {
    fn from((n, type_iri): (serde_json::Number, Option<DatatypeIRI>)) -> Self {
        Self::Number {
            number: n,
            type_iri,
        }
    }
}

impl TryFrom<f32> for Literal {
    type Error = ();

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        match serde_json::Number::from_f64(f as f64) {
            Some(f) => Ok(Self::Number {
                number: f,
                type_iri: well_known::xsd_float().into(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<time::OffsetDateTime> for Literal {
    type Error = time::error::Format;

    fn try_from(dt: time::OffsetDateTime) -> Result<Self, Self::Error> {
        let s = dt.format(&Rfc3339)?;
        Ok(Self::DateTime(s))
    }
}

impl From<(Vec<u8>, DatatypeIRI)> for Literal {
    fn from((data, type_iri): (Vec<u8>, DatatypeIRI)) -> Self {
        Self::Raw { data, type_iri }
    }
}

impl From<(&str, Lang)> for Literal {
    fn from((string, lang): (&str, Lang)) -> Self {
        Self::LangString {
            string: string.into(),
            lang,
        }
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}
impl From<bool> for LiteralOrIRI {
    fn from(b: bool) -> Self {
        Self::Literal(Literal::from(b))
    }
}
