use std::{fmt::Display, str::FromStr};

use super::{ClassIRI, ObjectPropertyIRI};

pub fn iri<T: From<IRI>>(iri: &str) -> T {
    IRI::new(iri).unwrap().into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IRI(iref::IriBuf);

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
        })
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
}

impl IRIBuilder {
    pub fn construct(iri: IRI) -> Self {
        Self { iribuf: iri.0 }
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
}
