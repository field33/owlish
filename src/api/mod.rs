mod ontology;
pub use ontology::*;

pub use crate::owl::IRI;
pub use crate::owl::Axiom;

#[cfg(feature = "wasm")] // TODO
pub mod wasm;
