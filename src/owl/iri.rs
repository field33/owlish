use std::{borrow::Cow, collections::HashMap, fmt::Display, str::FromStr};

use crate::error::Error;

use super::{ClassIRI, ObjectPropertyIRI};
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use serde_json::Value;

pub fn iri<T: From<IRI>>(iri: &str) -> T {
    IRI::new(iri).unwrap().into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl From<IRI> for Value {
    fn from(s: IRI) -> Self {
        Value::from(s.as_str())
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
}
