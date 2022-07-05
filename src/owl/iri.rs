use std::{borrow::Cow, collections::HashMap, fmt::Display, str::FromStr};

use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::Value;

use super::{ClassIRI, ObjectPropertyIRI};

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
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                IRI::new(value).map_err(|e| E::custom("Invalid IRI"))
            }
        }
        deserializer.deserialize_str(IRIVisitor)
    }
}

impl Serialize for IRI {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
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

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<iref::Error> for Error {
    fn from(e: iref::Error) -> Self {
        Error {
            message: e.to_string(),
        }
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
            None => match name {
                Some(name) => Some(self.new(name)),
                None => None,
            },
        }
    }
}

impl From<IRI> for Value {
    fn from(s: IRI) -> Self {
        Value::from(s.as_str())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ser_de() {
        let iri = IRI::new("https://test.org#asdf").unwrap();

        let json = serde_json::to_string(&iri).unwrap();

        assert_eq!(json, r#""https://test.org#asdf""#);

        let json = r#""https://test.org#asdf""#;
        let iri1: IRI = serde_json::from_str(json).unwrap();

        assert_eq!(iri1, iri)
    }
}
