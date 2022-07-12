use serde::{Deserialize, Serialize};

use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubObjectPropertyOf(pub ObjectPropertyConstructor, pub ObjectPropertyIRI);
