mod class;
pub use class::*;
mod ontology;
pub use ontology::*;
mod individual;
pub use individual::*;
#[cfg(feature = "wasm")] // TODO
pub mod wasm;
