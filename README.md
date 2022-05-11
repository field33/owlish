# owlib

This library provides OWL2 datastructures that allow you to build and work with ontologies.

The OWL2 model is based on functional style syntax. E.g. the function

```
ClassAssertion( :Person :Mary )
```

Is represented as a similar tuple struct

```rust
pub struct ClassAssertion(pub(crate) ClassConstructor, pub(crate) IndividualIRI);
```

## Usage

owlib provides two APIs:

1. A low level representation of OWL based on functional syntax
   - This is exported in `owlib::owl::*`
2. A conceptional field33-specific api that concatenates OWL data for relevant types.
   - This is exported in `owlib::api::*`

## Dev stuff

Build:

```
cargo build
```

Test:

```
cargo test
```

## Commits and Releases

This crate uses [convenentional commits](https://www.conventionalcommits.org/en/v1.0.0/) to create automated releases whenever the main branch is updated. In addition the CHANGELOG.md is automatically generated.
