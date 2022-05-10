use std::{fmt::Display, str::FromStr};

use super::ClassIRI;

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

// iri builder
#[derive(Debug, Clone)]
pub struct IRIBuilder {
    iribuf: iref::IriBuf,
}

impl IRIBuilder {
    pub fn new_checked<T: From<IRI>>(&self, name: &str) -> Result<T, Error> {
        let mut iribuf = self.iribuf.clone();
        iribuf.set_fragment(Some(iref::Fragment::try_from(name)?));
        Ok(T::from(IRI(iribuf)))
    }
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<IRI>>(&self, name: &str) -> T {
        let mut iribuf = self.iribuf.clone();
        iribuf.set_fragment(Some(iref::Fragment::try_from(name).unwrap()));
        T::from(IRI(iribuf))
    }
    pub fn class(&self, name: &str) -> ClassIRI {
        self.new(name)
    }
}
