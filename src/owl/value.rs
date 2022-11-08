use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;

use crate::owl::IRI;

use super::{well_known, DatatypeIRI, Lang};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LiteralOrIRI {
    IRI(IRI),
    Literal(Literal),
}

impl std::fmt::Display for LiteralOrIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralOrIRI::IRI(iri) => {
                write!(f, "{}", iri)
            }
            LiteralOrIRI::Literal(lit) => {
                write!(f, "{}", lit)
            }
        }
    }
}

impl From<Literal> for LiteralOrIRI {
    fn from(l: Literal) -> Self {
        Self::Literal(l)
    }
}

const KEY_LITERAL: &str = "Literal";
const KEY_IRI: &str = "IRI";

impl Serialize for LiteralOrIRI {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        match self {
            LiteralOrIRI::Literal(value) => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(KEY_LITERAL)?;

                map.serialize_key(KEY_LITERAL)?;
                map.serialize_value(value)?;
            }
            LiteralOrIRI::IRI(value) => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(KEY_IRI)?;

                map.serialize_key(KEY_IRI)?;
                map.serialize_value(value)?;
            }
        };
        map.end()
    }
}

impl<'de> Deserialize<'de> for LiteralOrIRI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LiteralOrIRIVisitor;
        impl<'de> Visitor<'de> for LiteralOrIRIVisitor {
            type Value = LiteralOrIRI;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"an object {_type: <"Literal" | "IRI">, <IRI | Literal>: <the iri or literal string> }"#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut pairs = Vec::new();

                let key1: Option<&str> = map.next_key()?;
                let value1: Option<serde_json::Value> = map.next_value()?;
                if let (Some(k), Some(v)) = (key1, value1) {
                    pairs.push((k, v));
                }
                let key2: Option<&str> = map.next_key()?;
                let value2: Option<serde_json::Value> = map.next_value()?;
                if let (Some(k), Some(v)) = (key2, value2) {
                    pairs.push((k, v));
                }

                if pairs.len() < 2 {
                    return Err(serde::de::Error::custom(
                        "Could not parse Value: Expected 2 fields bug got less.",
                    ));
                }

                if let Some((_, _type)) = pairs.iter().find(|(k, _)| *k == KEY_TYPE) {
                    let _type = _type.as_str().ok_or_else(|| {
                        serde::de::Error::custom(
                            "Could not parse Value: Key '_type' is not a string.",
                        )
                    })?;

                    match _type {
                        KEY_IRI => {
                            if let Some(iri) = pairs
                                .iter()
                                .find(|(k, _)| *k == KEY_IRI)
                                .and_then(|(_, v)| v.as_str())
                            {
                                match IRI::new(iri) {
                                    Ok(iri) => return Ok(LiteralOrIRI::IRI(iri)),
                                    Err(_) => {
                                        return Err(serde::de::Error::custom("Could not parse IRI"))
                                    }
                                }
                            }
                        }
                        KEY_LITERAL => {
                            if let Some((_, value)) =
                                pairs.into_iter().find(|(k, _)| *k == KEY_LITERAL)
                            {
                                let value: Literal =
                                    serde_json::from_value(value).map_err(|_| {
                                        serde::de::Error::custom(
                                            "Could not parse value for LiteralOrValue",
                                        )
                                    })?;
                                return Ok(LiteralOrIRI::Literal(value));
                            }
                        }
                        _ => {
                            return Err(serde::de::Error::custom("Could not parse LiteralOrIRI."));
                        }
                    }
                }
                Err(serde::de::Error::custom("Could not parse LiteralOrIRI."))
            }
        }
        deserializer.deserialize_map(LiteralOrIRIVisitor)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Raw { data, type_iri } => write!(f, "{}: {:?}", type_iri.as_iri(), data),
            Literal::String(s) => write!(f, "{}", s),
            Literal::DateTime(dt) => write!(f, "{}", dt),
            Literal::LangString { string, lang } => write!(f, "{}@{}", string, lang),
            Literal::Number {
                number,
                type_iri: _,
            } => write!(f, "{}", number,),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}

const VALUE_STRING_TYPE: &str = "string";
const VALUE_RAW_TYPE: &str = "raw";
const VALUE_DATETIME_TYPE: &str = "dateTime";
const VALUE_LANG_STRING_TYPE: &str = "langString";
const VALUE_NUMBER_TYPE: &str = "number";
const VALUE_BOOLEAN_TYPE: &str = "boolean";

const KEY_TYPE: &str = "_type";
const KEY_DATATYPE: &str = "datatypeIRI";
const KEY_VALUE: &str = "value";
const KEY_LANG: &str = "lang";

impl Serialize for Literal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        match self {
            Literal::String(value) => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(VALUE_STRING_TYPE)?;

                map.serialize_key(KEY_DATATYPE)?;
                map.serialize_value(well_known::xsd_string_str)?;

                map.serialize_key(KEY_VALUE)?;
                map.serialize_value(value)?;

                map.serialize_key(KEY_LANG)?;
                map.serialize_value(&serde_json::Value::Null)?;
            }
            Literal::Raw { data, type_iri } => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(VALUE_RAW_TYPE)?;

                map.serialize_key(KEY_DATATYPE)?;
                map.serialize_value(type_iri.as_iri().as_str())?;

                map.serialize_key(KEY_VALUE)?;
                map.serialize_value(data)?;

                map.serialize_key(KEY_LANG)?;
                map.serialize_value(&serde_json::Value::Null)?;
            }
            Literal::DateTime(date) => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(VALUE_DATETIME_TYPE)?;

                map.serialize_key(KEY_DATATYPE)?;
                map.serialize_value(well_known::xsd_dateTime_str)?;

                map.serialize_key(KEY_VALUE)?;
                map.serialize_value(date)?;

                map.serialize_key(KEY_LANG)?;
                map.serialize_value(&serde_json::Value::Null)?;
            }
            Literal::LangString {
                string: value,
                lang,
            } => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(VALUE_LANG_STRING_TYPE)?;

                map.serialize_key(KEY_DATATYPE)?;
                map.serialize_value(well_known::xsd_string_str)?;

                map.serialize_key(KEY_VALUE)?;
                map.serialize_value(value)?;

                map.serialize_key(KEY_LANG)?;
                map.serialize_value(lang.string())?;
            }
            Literal::Number { number, type_iri } => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(VALUE_NUMBER_TYPE)?;

                if let Some(type_iri) = type_iri {
                    map.serialize_key(KEY_DATATYPE)?;
                    map.serialize_value(type_iri.as_iri().as_str())?;
                } else {
                    map.serialize_key(KEY_DATATYPE)?;
                    map.serialize_value(well_known::xsd_float_str)?;
                }

                map.serialize_key(KEY_VALUE)?;
                map.serialize_value(number)?;

                map.serialize_key(KEY_LANG)?;
                map.serialize_value(&serde_json::Value::Null)?;
            }
            Literal::Bool(value) => {
                map.serialize_key(KEY_TYPE)?;
                map.serialize_value(VALUE_BOOLEAN_TYPE)?;

                map.serialize_key(KEY_DATATYPE)?;
                map.serialize_value(well_known::xsd_boolean_str)?;

                map.serialize_key(KEY_VALUE)?;
                map.serialize_value(value)?;

                map.serialize_key(KEY_LANG)?;
                map.serialize_value(&serde_json::Value::Null)?;
            }
        };
        map.end()
    }
}

impl<'de> Deserialize<'de> for Literal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Literal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"an object {_type: "IRI", value: <the iri string> }"#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut pairs = Vec::new();

                let key1: Option<&str> = map.next_key()?;
                let value1: Option<serde_json::Value> = map.next_value()?;
                if let (Some(k), Some(v)) = (key1, value1) {
                    pairs.push((k, v));
                }
                let key2: Option<&str> = map.next_key()?;
                let value2: Option<serde_json::Value> = map.next_value()?;
                if let (Some(k), Some(v)) = (key2, value2) {
                    pairs.push((k, v));
                }

                let key3: Option<&str> = map.next_key()?;
                let value3: Option<serde_json::Value> = map.next_value()?;
                if let (Some(k), Some(v)) = (key3, value3) {
                    pairs.push((k, v));
                }
                let key4: Option<&str> = map.next_key()?;
                let value4: Option<serde_json::Value> = map.next_value()?;
                if let (Some(k), Some(v)) = (key4, value4) {
                    pairs.push((k, v));
                }

                if pairs.len() < 3 {
                    return Err(serde::de::Error::custom(
                        "Could not parse Value: Expected 3 fields bug got less.",
                    ));
                }

                let literal: Literal;

                if let Some((_, datatype_iri)) = pairs.iter().find(|(k, _)| *k == KEY_DATATYPE) {
                    if let Some((_, value)) = pairs.iter().find(|(k, _)| *k == KEY_VALUE) {
                        if let Some((_, _type)) = pairs.iter().find(|(k, _)| *k == KEY_TYPE) {
                            let _type = _type.as_str().ok_or_else(|| {
                                serde::de::Error::custom(
                                    "Could not parse Value: Key '_type' is not a string.",
                                )
                            })?;
                            match _type {
                                VALUE_STRING_TYPE => {
                                    let value = value.as_str().ok_or_else(|| serde::de::Error::custom(
                                        "Could not parse Value: Key 'value' is expected to be a string.",
                                    ))?;
                                    literal = Literal::String(value.into());
                                }
                                VALUE_RAW_TYPE => {
                                    let datatype_iri = datatype_iri
                                        .as_str()
                                        .ok_or_else(|| serde::de::Error::custom(
                                        "Could not parse Value: Key 'datatypeIRI' is not a string.",
                                    ))?;
                                    let datatype_iri = IRI::new(datatype_iri).map_err(|_| serde::de::Error::custom(
                                        "Could not parse Value: Key 'datatypeIRI' is not a valid IRI.",
                                    ))?;

                                    let mut data = Vec::new();
                                    let value = value.as_array().ok_or_else(|| serde::de::Error::custom(
                                        "Could not parse Value: Key 'value' is expected to be an u8 array.",
                                    ))?;
                                    for v in value {
                                        let value = v.as_u64().ok_or_else( || serde::de::Error::custom(
                                            "Could not parse Value: Key 'value' is expected to be an u8 array.",
                                        ))?;
                                        data.push(value as u8);
                                    }
                                    literal = Literal::Raw {
                                        data,
                                        type_iri: datatype_iri.into(),
                                    }
                                }
                                VALUE_DATETIME_TYPE => {
                                    let value = value.as_str().ok_or_else(||serde::de::Error::custom(
                                        "Could not parse Value: Key 'value' is expected to be a string.",
                                    ))?;
                                    literal = Literal::DateTime(value.into());
                                }
                                VALUE_LANG_STRING_TYPE => {
                                    let value = value.as_str().ok_or_else(|| serde::de::Error::custom(
                                        "Could not parse Value: Key 'value' is expected to be a string.",
                                    ))?;

                                    if let Some((_, lang)) =
                                        pairs.iter().find(|(k, _)| *k == KEY_LANG)
                                    {
                                        let lang = lang.as_str().ok_or_else(|| serde::de::Error::custom(
                                            "Could not parse Value: Key 'lang' is expected to be a string.",
                                        ))?;

                                        let lang = lang.try_into().map_err(|_| serde::de::Error::custom(
                                            "Could not parse Value: Key 'value' is not a valid language key.",
                                        ))?;

                                        literal = Literal::LangString {
                                            string: value.into(),
                                            lang,
                                        };
                                    } else {
                                        literal = Literal::LangString {
                                            string: value.into(),
                                            lang: Lang::EN,
                                        };
                                    }
                                }
                                VALUE_NUMBER_TYPE => {
                                    let datatype_iri = datatype_iri
                                        .as_str()
                                        .ok_or_else(|| serde::de::Error::custom(
                                        "Could not parse Value: Key 'datatypeIRI' is not a string.",
                                    ))?;
                                    let datatype_iri = IRI::new(datatype_iri).map_err(|_| serde::de::Error::custom(
                                        "Could not parse Value: Key 'datatypeIRI' is not a valid IRI.",
                                    ))?;

                                    match value {
                                        serde_json::Value::Number(number) => {
                                            literal = Literal::Number {
                                                number: number.clone(),
                                                type_iri: Some(datatype_iri.into()),
                                            }
                                        }
                                        _ => {
                                            return Err(
                                                serde::de::Error::custom(
                                                    "Could not parse Value: Expected numeric type but got something else.",
                                                )
                                            )
                                        }
                                    }
                                }
                                VALUE_BOOLEAN_TYPE => {
                                    match value {
                                        serde_json::Value::Bool(b) => {
                                            literal = Literal::Bool(*b);
                                        },
                                        _ => {
                                            return Err(
                                                serde::de::Error::custom(
                                                    "Could not parse Value: Expected boolean type but got something else.",
                                                )
                                            )
                                        }
                                    }
                                },
                                v => {
                                    return Err(serde::de::Error::custom(
                                        format!("Could not parse Value: Unknown '_type' value: '{:?}'.", v),
                                    ));
                                }
                            }
                        } else {
                            return Err(serde::de::Error::custom(
                                "Could not parse Value: Missing key '_type'.",
                            ));
                        }
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse Value: Missing key 'value'.",
                        ));
                    }
                } else {
                    return Err(serde::de::Error::custom(
                        "Could not parse Value: Missing key 'datatypeIRI'.",
                    ));
                }
                Ok(literal)
            }
        }
        deserializer.deserialize_map(ValueVisitor)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let lit = Literal::String("test".into());
        assert_eq!(
            serde_json::to_string(&lit).unwrap(),
            r#"{"_type":"string","datatypeIRI":"http://www.w3.org/2001/XMLSchema#string","value":"test","lang":null}"#
        );
        let lit = Literal::Raw {
            data: vec![1, 2, 3, 4],
            type_iri: well_known::xsd_float(),
        };
        assert_eq!(
            serde_json::to_string(&lit).unwrap(),
            r#"{"_type":"raw","datatypeIRI":"http://www.w3.org/2001/XMLSchema#float","value":[1,2,3,4],"lang":null}"#
        )
    }
    #[test]
    fn test_deserialize() {
        let lit = r#"{"_type":"string","datatypeIRI":"http://www.w3.org/2001/XMLSchema#string","value":"test","lang":null}"#;
        assert_eq!(
            serde_json::from_str::<Literal>(lit).unwrap(),
            Literal::String("test".into())
        );
        let lit = r#"{"_type":"raw","datatypeIRI":"http://www.w3.org/2001/XMLSchema#float","value":[1,2,3,4],"lang":null}"#;
        assert_eq!(
            serde_json::from_str::<Literal>(lit).unwrap(),
            Literal::Raw {
                data: vec![1, 2, 3, 4],
                type_iri: well_known::xsd_float(),
            }
        )
    }
}
