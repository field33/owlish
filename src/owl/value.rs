use crate::owl::IRI;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    String(String),
    Integer(i64),
    NonNegativeInteger(u64),
    IRI(IRI),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.into())
    }
}
impl From<IRI> for Value {
    fn from(iri: IRI) -> Self {
        Self::IRI(iri)
    }
}

impl Value {
    pub fn string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}
