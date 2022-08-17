mod iri;
pub use iri::*;

pub mod well_known;

mod value;
pub use value::*;

mod classes;
pub use classes::*;

mod individual;
pub use individual::*;

mod datatypes;
pub use datatypes::*;

mod ontology;
pub use ontology::*;

mod axiom;
pub use axiom::*;

mod properties;
pub use properties::*;

mod other;
pub use other::*;

mod regards;
pub use regards::*;

mod lang;
pub use lang::*;
