use harriet::triple_production::RdfBlankNode;
use std::{borrow::Cow, collections::HashMap, fmt::Display, str::FromStr};

use crate::error::Error;

use super::{ClassIRI, ObjectPropertyIRI};
use iref::Fragment;
use pct_str::{PctString, URIReserved};
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use snowflake::ProcessUniqueId;

pub fn iri<T: From<IRI>>(iri: &str) -> T {
    IRI::new(iri).unwrap().into()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IRI(iref::IriBuf);

impl<'de> Deserialize<'de> for IRI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IRIVisitor;
        impl<'de> Visitor<'de> for IRIVisitor {
            type Value = IRI;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"an object {_type: "IRI", value: <the iri string> }"#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let key1: Option<&str> = map.next_key()?;
                let value1: Option<&str> = map.next_value()?;
                let key2: Option<&str> = map.next_key()?;
                let value2: Option<&str> = map.next_value()?;

                if let (Some("string"), Some(iri)) = (key1, value1) {
                    IRI::try_from(iri).map_err(|_| {
                        serde::de::Error::custom(format!("Could not parse IRI {}", iri))
                    })
                } else if let (Some("string"), Some(iri)) = (key2, value2) {
                    IRI::try_from(iri).map_err(|_| {
                        serde::de::Error::custom(format!("Could not parse IRI {}", iri))
                    })
                } else {
                    Err(serde::de::Error::custom(
                        "Could not parse IRI: Missing 'value' field.",
                    ))
                }
            }
        }
        deserializer.deserialize_map(IRIVisitor)
    }
}

impl Serialize for IRI {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_key("_type")?;
        map.serialize_value("IRI")?;
        map.serialize_key("string")?;
        map.serialize_value(self.0.as_str())?;
        map.end()
    }
}

impl Display for IRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IRI {
    pub fn new(iri: &str) -> Result<Self, Error> {
        Ok(iref::IriBuf::from_str(iri).map(Self)?)
    }

    pub fn builder(prefix: &str) -> Result<IRIBuilder, Error> {
        Ok(IRIBuilder {
            iribuf: iref::IriBuf::from_str(prefix)?,
            imports: Default::default(),
        })
    }

    pub fn set_leaf(&mut self, new_leaf_name: Option<String>) -> Result<(), String> {
        if self.0.fragment().is_some() {
            if let Some(frag) = new_leaf_name {
                let frag = Fragment::try_from(frag.as_str())
                    .map_err(|e| format!("Failed to set iri fragment: {}", e))?;
                self.0.set_fragment(Some(frag));
            } else {
                self.0.set_fragment(None);
            }
        } else if let Some(name) = new_leaf_name {
            let temp_iri = iref::IriBuf::from_string(format!("http://a/{}", name))
                .map_err(|e| format!("Failed to parse given iri segment: {:?}", e))?;
            self.0.path_mut().pop();
            if let Some(seg) = temp_iri.path().segments().last() {
                self.0.path_mut().push(seg);
            } else {
                return Err("Failed to set last segment.".to_string());
            }
        }
        Ok(())
    }

    pub fn set_query_parameter(&mut self, name: &str, value: &str) -> Result<(), String> {
        match self.0.query() {
            Some(q) => {
                let v = PctString::encode(value.chars(), URIReserved);
                let query_str = format!("{}&{}={}", q.as_str(), name, v.as_str());
                match iref::Query::try_from(query_str.as_str()) {
                    Ok(q) => {
                        self.0.set_query(Some(q));
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to set iri query '{}': {}", query_str, e)),
                }
            }
            None => {
                let v = PctString::encode(value.chars(), URIReserved);
                let query_str = format!("{}={}", name, v);
                match iref::Query::try_from(query_str.as_str()) {
                    Ok(q) => {
                        self.0.set_query(Some(q));
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to set iri query '{}': {}", query_str, e)),
                }
            }
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for IRI {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(iref::IriBuf::from_str(value).map(Self)?)
    }
}

/// Allows to create IRIs for suffixes (IRI fragments).
#[derive(Debug, Clone)]
pub struct IRIBuilder {
    iribuf: iref::IriBuf,
    imports: HashMap<String, iref::IriBuf>,
}

impl IRIBuilder {
    pub fn construct(iri: IRI, imports: &HashMap<String, IRI>) -> Self {
        Self {
            iribuf: iri.0,
            imports: imports
                .iter()
                .map(|(name, iri)| (name.clone(), iri.0.clone()))
                .collect(),
        }
    }

    /// Get the base IRI of this builder
    pub fn base(&self) -> IRI {
        IRI(self.iribuf.clone())
    }

    /// Create a new IRI from the given name.
    /// The name will be used as fragment of the returned IRI.
    /// Checks for validity of the IRI.
    pub fn new_checked<T: From<IRI>>(&self, name: &str) -> Result<T, Error> {
        let mut iribuf = self.iribuf.clone();
        iribuf.set_fragment(Some(iref::Fragment::try_from(name)?));
        Ok(T::from(IRI(iribuf)))
    }

    /// Create a new IRI from the given name.
    /// The name will be used as fragment of the returned IRI.
    /// Panics if the name is not a valid IRI fragment.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<IRI>>(&self, name: &str) -> T {
        let mut iribuf = self.iribuf.clone();
        iribuf.set_fragment(Some(iref::Fragment::try_from(name).unwrap()));
        T::from(IRI(iribuf))
    }

    /// Create a new ClassIRI from the given name.
    /// The name will be used as fragment of the returned IRI.
    /// Panics if the name is not a valid IRI fragment.
    pub fn class(&self, name: &str) -> ClassIRI {
        self.new(name)
    }
    /// Create a new ObjectPropertyIRI from the given name.
    /// The name will be used as fragment of the returned IRI.
    /// Panics if the name is not a valid IRI fragment.
    pub fn object_prop(&self, name: &str) -> ObjectPropertyIRI {
        self.new(name)
    }

    pub fn from<T: From<IRI>>(&self, prefix: &str, name: &str) -> Option<T> {
        match self.imports.get(prefix) {
            Some(prefix) => {
                let mut iribuf = prefix.clone();
                iribuf.set_fragment(Some(iref::Fragment::try_from(name).unwrap()));
                Some(T::from(IRI(iribuf)))
            }
            None => None,
        }
    }

    pub fn from_opt<T: From<IRI>>(
        &self,
        prefix: &Option<Cow<str>>,
        name: &Option<Cow<str>>,
    ) -> Option<T> {
        match prefix {
            Some(prefix) => match name {
                Some(name) => match self.imports.get(prefix.as_ref()) {
                    Some(prefix) => {
                        let mut iribuf = prefix.clone();
                        iribuf.set_fragment(Some(iref::Fragment::try_from(name.as_ref()).unwrap()));
                        Some(T::from(IRI(iribuf)))
                    }
                    None => {
                        let mut iribuf = self.iribuf.clone();
                        iribuf.set_fragment(Some(iref::Fragment::try_from(name.as_ref()).unwrap()));
                        Some(T::from(IRI(iribuf)))
                    }
                },
                None => None,
            },
            None => name.as_ref().map(|name| self.new(name)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlankNode(snowflake::ProcessUniqueId);

impl BlankNode {
    pub fn new() -> Self {
        Self(snowflake::ProcessUniqueId::new())
    }
}

impl From<RdfBlankNode> for BlankNode {
    fn from(value: RdfBlankNode) -> Self {
        Self(value.internal_id)
    }
}

impl Serialize for BlankNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_key("_type")?;
        map.serialize_value("BlankNode")?;
        map.serialize_key("inner")?;
        map.serialize_value(&self.0)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for BlankNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IRIVisitor;
        impl<'de> Visitor<'de> for IRIVisitor {
            type Value = BlankNode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"an object {_type: "BlankNode", inner: <the object for the process-uniquer internal id> }"#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let key1: Option<&str> = map.next_key()?;
                if let Some("inner") = key1 {
                    let value: Option<ProcessUniqueId> = map.next_value()?;
                    if let Some(inner_id) = value {
                        return Ok(BlankNode(inner_id));
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse BlankNode: Malformed 'inner' field.",
                        ));
                    }
                } else {
                    let _value: Option<&str> = map.next_value()?;
                }

                let key2: Option<&str> = map.next_key()?;
                if let Some("inner") = key2 {
                    let value: Option<ProcessUniqueId> = map.next_value()?;
                    if let Some(inner_id) = value {
                        return Ok(BlankNode(inner_id));
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse BlankNode: Malformed 'inner' field.",
                        ));
                    }
                }
                Err(serde::de::Error::custom(
                    "Could not parse BlankNode: Missing 'inner' field.",
                ))
            }
        }
        deserializer.deserialize_map(IRIVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceId {
    IRI(IRI),
    BlankNode(BlankNode),
}

impl ResourceId {
    pub fn is_iri(&self) -> bool {
        matches!(&self, Self::IRI(_))
    }

    pub fn is_blank_node(&self) -> bool {
        matches!(&self, Self::BlankNode(_))
    }
}

impl From<IRI> for ResourceId {
    fn from(value: IRI) -> Self {
        Self::IRI(value)
    }
}

impl From<BlankNode> for ResourceId {
    fn from(value: BlankNode) -> Self {
        Self::BlankNode(value)
    }
}

impl Serialize for ResourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_key("_type")?;
        map.serialize_value("ResourceId")?;
        match self {
            ResourceId::IRI(iri) => {
                map.serialize_key("iri")?;
                map.serialize_value(&iri)?;
            }
            ResourceId::BlankNode(blank_node) => {
                map.serialize_key("blankNode")?;
                map.serialize_value(&blank_node)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for ResourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IRIVisitor;
        impl<'de> Visitor<'de> for IRIVisitor {
            type Value = ResourceId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    r#"an object {_type: "ResourceId", either with key `iri` or `blankNode` }"#,
                )
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let key1: Option<&str> = map.next_key()?;
                if let Some("iri") = key1 {
                    let value: Option<IRI> = map.next_value()?;
                    if let Some(inner_id) = value {
                        return Ok(ResourceId::IRI(inner_id));
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse BlankNode: Malformed 'iri' field.",
                        ));
                    }
                } else if let Some("blankNode") = key1 {
                    let value: Option<BlankNode> = map.next_value()?;
                    if let Some(inner_id) = value {
                        return Ok(ResourceId::BlankNode(inner_id));
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse BlankNode: Malformed 'blankNode' field.",
                        ));
                    }
                } else {
                    let _value: Option<&str> = map.next_value()?;
                }

                let key2: Option<&str> = map.next_key()?;
                if let Some("iri") = key2 {
                    let value: Option<IRI> = map.next_value()?;
                    if let Some(inner_id) = value {
                        return Ok(ResourceId::IRI(inner_id));
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse BlankNode: Malformed 'iri' field.",
                        ));
                    }
                } else if let Some("blankNode") = key2 {
                    let value: Option<BlankNode> = map.next_value()?;
                    if let Some(inner_id) = value {
                        return Ok(ResourceId::BlankNode(inner_id));
                    } else {
                        return Err(serde::de::Error::custom(
                            "Could not parse BlankNode: Malformed 'blankNode' field.",
                        ));
                    }
                } else {
                    let _value: Option<&str> = map.next_value()?;
                }

                Err(serde::de::Error::custom(
                    "Could not parse ResourceId: Missing 'iri' or `blankNode` field.",
                ))
            }
        }
        deserializer.deserialize_map(IRIVisitor)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const ONTOLOGY_TS_API: &'static str = r#"
export interface IRI {
    _type: "IRI",
    string: string
}

export function Iri(iri: string): IRI

export interface BlankNode {
    _type: "BlankNode"
}

export interface ResourceId {
    _type: "ResourceId",
    iri?: IRI,
    blankNode?: BlankNode,
}
"#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ser_de() {
        let iri = IRI::new("https://test.org#asdf").unwrap();

        let json = serde_json::to_string(&iri).unwrap();

        assert_eq!(json, r#"{"_type":"IRI","string":"https://test.org#asdf"}"#);

        let json = r#"{"_type":"IRI","string":"https://test.org#asdf"}"#;
        let iri1: IRI = serde_json::from_str(json).unwrap();

        assert_eq!(iri1, iri)
    }

    #[test]
    pub fn test_iri_parameter() {
        let mut iri = IRI::new("https://test.org#asdf").unwrap();
        iri.set_query_parameter("test", "bla").unwrap();
        assert_eq!(iri.as_str(), "https://test.org?test=bla#asdf");
        iri.set_query_parameter("foo", "bar").unwrap();
        assert_eq!(iri.as_str(), "https://test.org?test=bla&foo=bar#asdf");
        iri.set_query_parameter("foo", "https://test.org#foo?test=false")
            .unwrap();
        assert_eq!(iri.as_str(), "https://test.org?test=bla&foo=bar&foo=https%3A%2F%2Ftest.org%23foo%3Ftest%3Dfalse#asdf");
    }

    #[test]
    pub fn test_iri_parameter2() {
        let mut iri = IRI::new("https://test.org#asdf").unwrap();
        iri.set_query_parameter("source", "http://field33.com/dataset/p#11")
            .unwrap();
        assert_eq!(
            iri.as_str(),
            "https://test.org?source=http%3A%2F%2Ffield33.com%2Fdataset%2Fp%2311#asdf"
        );
    }

    #[test]
    pub fn test_set_leaf() {
        let mut iri = IRI::new("https://test.org#asdf").unwrap();

        iri.set_leaf(Some("Foobar".into())).unwrap();
        assert_eq!(iri.as_str(), "https://test.org#Foobar");

        let mut iri = IRI::new("https://test.org/asdf").unwrap();

        iri.set_leaf(Some("Foobar".into())).unwrap();
        assert_eq!(iri.as_str(), "https://test.org/Foobar");
    }

    #[test]
    pub fn test_ser_de_blank_node() {
        let blank_node_1 = BlankNode::new();
        let blank_node_2 = BlankNode::new();

        let json = serde_json::to_string(&blank_node_1).unwrap();

        // Rough checking, as this has a variable part inside of it
        assert!(json.contains(r#"{"_type":"BlankNode","inner":{"prefix":"#));

        dbg!(&json);
        let parsed_blank_node: BlankNode = serde_json::from_str(&json).unwrap();

        assert_eq!(blank_node_1, parsed_blank_node);
        assert_ne!(blank_node_2, parsed_blank_node)
    }

    #[test]
    pub fn test_ser_de_resource_id() {
        let iri = ResourceId::IRI(IRI::new("https://test.org#asdf").unwrap());

        let json = serde_json::to_string(&iri).unwrap();

        assert_eq!(
            json,
            r#"{"_type":"ResourceId","iri":{"_type":"IRI","string":"https://test.org#asdf"}}"#
        );

        let json =
            r#"{"_type":"ResourceId","iri":{"_type":"IRI","string":"https://test.org#asdf"}}"#;
        let iri1: ResourceId = serde_json::from_str(json).unwrap();

        assert_eq!(iri1, iri)
    }
}
