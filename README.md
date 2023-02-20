# owlish

[<img alt="github" src="https://img.shields.io/badge/github-field33/owlish-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/field33/owlish)
[<img alt="crates.io" src="https://img.shields.io/crates/v/owlish.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/owlish)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-owlish-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/owlish)
[<img alt="npmjs.com" src="https://img.shields.io/npm/v/owlish.svg?style=for-the-badge&color=fc8d62&logo=npm" height="20">](https://www.npmjs.com/package/owlish)

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

owlish provides two APIs:

1. A low level representation of OWL based on functional syntax
   - This is exported in `owlish::owl::*`
2. A conceptional api that concatenates OWL data for relevant types.
   - TBD

## Usage (Node.js)

To initialize the module in a Node.js environment, it is currently recommend to load the WASM module via the `fs` API and
pass it explicitly to the initialization function.

Example:
```js
import path from 'path';
import { readFile } from 'fs/promises';
import { fileURLToPath } from 'url';

// The next two lines are only required if running the Node script as an ESM module
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
// Load .wasm file from the package
const owlishWasm = await readFile(path.join(__dirname, "../node_modules/owlish/owlish_bg.wasm"));
// Initialize module, after executing this line, all functions from `owlish` can be used like normal.
await owlish(owlishWasm)
```

## Dev stuff

Build:

```
cargo build
```

Test:

```
cargo test
```

Run benchmark tests:

```
cargo bench
```

## Commits and Releases

This crate uses [convenentional commits](https://www.conventionalcommits.org/en/v1.0.0/) to create automated releases whenever the main branch is updated. In addition the CHANGELOG.md is automatically generated.
