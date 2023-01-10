# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.20.1 - 2023-01-10
#### Bug Fixes
- Matcher implementations now return values - (983b890) - Florian Loers

- - -

## v0.20.0 - 2023-01-03
#### Features
- Parse reification iris as annotations - (adc3b9b) - Florian Loers
- Add ttl serialization - (775cebf) - Florian Loers

- - -

## v0.19.1 - 2022-12-22
#### Bug Fixes
- Fix error in public type generation - (dd85aed) - Florian Loers

- - -

## v0.19.0 - 2022-12-19
#### Features
- Parse EquivalentOfClasses axioms - (0e99c1d) - Florian Loers

- - -

## v0.18.1 - 2022-12-12
#### Bug Fixes
- Fix parsing UnionOf constructs - (5e75c47) - Florian Loers

- - -

## v0.18.0 - 2022-12-09
#### Features
- Parse Sub*PropertyOf axioms - (2d58473) - Florian Loers

- - -

## v0.17.7 - 2022-11-25
#### Bug Fixes
- Parse unordered sequences - (6b80f43) - Florian Loers

- - -

## v0.17.6 - 2022-11-24
#### Bug Fixes
- Remove print command in parser - (f7fc3d4) - Florian Loers

- - -

## v0.17.5 - 2022-11-23
#### Bug Fixes
- Parse AnnotationProperties that have been defined as DataProperties as well - (ac8b229) - Florian Loers

- - -

## v0.17.4 - 2022-11-22
#### Bug Fixes
- Fix wrong matchDeclaration typescript types - (be08ecd) - Florian Loers

- - -

## v0.17.3 - 2022-11-22
#### Bug Fixes
- Fix wrong Declaration typescript types - (12e6c9a) - Florian Loers

- - -

## v0.17.2 - 2022-11-22
#### Bug Fixes
- Parsing annotations on data properties did not work correctly - (5d50e9d) - Florian Loers
#### Miscellaneous Chores
- Cleanup types - (66541f0) - Florian Loers

- - -

## v0.17.1 - 2022-11-08
#### Bug Fixes
- Parsing annotations on data properties did not work correctly - (012d428) - Florian Loers

- - -

## v0.17.0 - 2022-11-08
#### Features
- Parse annotations on data properties - (f8ed0ad) - Florian Loers

- - -

## v0.16.0 - 2022-11-04
#### Features
- Allow to combine ontologies via Ontology.append - (8ca2ca6) - Florian Loers

- - -

## v0.15.1 - 2022-11-04
#### Bug Fixes
- ClassAssertion has correct class and individual IRIs - (fa44128) - Kate Sieraia
#### Miscellaneous Chores
- Fix tests - (90fa10a) - Kate Sieraia

- - -

## v0.15.0 - 2022-11-01
#### Features
- Support parsing UnionOf constructs in ObjectPropertyDomain and ObjectPropertyRange constructs - (9fded31) - Florian Loers

- - -

## v0.14.0 - 2022-10-20
#### Features
- Support parsing ObjectPropertyRange and ObjectPropertyDomain for simple cases - (ecd1e55) - Florian Loers

- - -

## v0.13.0 - 2022-10-20
#### Features
- Support parsing ObjectProperties - (4f1c8c8) - Florian Loers

- - -

## v0.12.1 - 2022-10-18
#### Bug Fixes
- Fix wrong well_known type export - (4781314) - Florian Loers

- - -

## v0.12.0 - 2022-10-14
#### Features
- Parse unknown Data Property Assertions - (7949d3e) - Kate Sieraia

- - -

## v0.11.1 - 2022-10-13
#### Bug Fixes
- Implement Hash for IRI - (87da92a) - Florian Loers

- - -

## v0.11.0 - 2022-10-07
#### Features
- Parse numeric and bolean literals - (96b710b) - Florian Loers

- - -

## v0.10.0 - 2022-10-06
#### Features
- Parse object property domain and range - (625e5f4) - Florian Loers

- - -

## v0.9.0 - 2022-10-06
#### Features
- Parse annotation assertions with IRIs as values - (5a1bc60) - Florian Loers

- - -

## v0.8.12 - 2022-10-05
#### Bug Fixes
- Adjust types - (9142426) - Florian Loers

- - -

## v0.8.11 - 2022-10-05
#### Bug Fixes
- Improve parser error messages - (cdce6c3) - Florian Loers

- - -

## v0.8.10 - 2022-10-05
#### Bug Fixes
- Improve parser performance - (d9eabce) - Florian Loers
#### Miscellaneous Chores
- Add benchmark tests - (bfe14ed) - Florian Loers

- - -

## v0.8.9 - 2022-09-26
#### Bug Fixes
- Fix wrong typescript IRI type - (3f4feb0) - Florian Loers

- - -

## v0.8.8 - 2022-09-26
#### Bug Fixes
- Fix wrong typescript types and add CI checks for that - (b0f689c) - Florian Loers

- - -

## v0.8.7 - 2022-09-26
#### Bug Fixes
- Fix wrong types in matchDeclaration - (9b18971) - Florian Loers
#### Miscellaneous Chores
- Add keywords and URLs to Cargo.toml - (a7a0fb8) - Maximilian Goisser
- Add badges to README - (4b533fe) - Maximilian Goisser

- - -

## v0.8.6 - 2022-09-19
#### Bug Fixes
- Bump harriet to allow parsing of utf strings - (426fabb) - Florian Loers

- - -

## v0.8.5 - 2022-09-19
#### Bug Fixes
- Parse triples in any order - (433ae93) - Florian Loers

- - -

## v0.8.4 - 2022-09-16
#### Bug Fixes
- Improve parser performance - (00aecbf) - Florian Loers
#### Miscellaneous Chores
- Improve logging and add more tests - (982b9a3) - Florian Loers

- - -

## v0.8.3 - 2022-09-14
#### Bug Fixes
- RDF Parser ignores order of triples - (ae037d6) - Florian Loers

- - -

## v0.8.2 - 2022-09-14
#### Bug Fixes
- Patch package.json in release action - (c283249) - Maximilian Goisser

- - -

## v0.8.1 - 2022-09-14
#### Bug Fixes
- Improve owlish parser performance - (f5e8467) - Florian Loers

- - -

## v0.8.0 - 2022-09-13
#### Features
- Improve owlish parser for computation logic - (47b73b3) - Florian Loers

- - -

## v0.7.0 - 2022-09-08
#### Features
- Add spec-based rdf triple parser - (dd00a40) - Florian Loers

- - -

## v0.6.2 - 2022-09-01
#### Bug Fixes
- Improve typescript types - (8c52359) - Florian Loers
#### Miscellaneous Chores
- Remove wee alloc - (e10a230) - Florian Loers

- - -

## v0.6.1 - 2022-08-17
#### Bug Fixes
- Fix broken wasm API for Iri handling - (cf6f89d) - Florian Loers

- - -

## v0.6.0 - 2022-08-17
#### Features
- Provide Iri constructor with validation - (2292248) - Florian Loers
- Allow annotations for Declarations - (011697c) - Florian Loers

- - -

## v0.5.0 - 2022-08-12
#### Features
- Support annotations in ObjectProperty, DataProperty and AnnotationProperty - (e3d84c3) - Florian Loers

- - -

## v0.4.3 - 2022-08-08
#### Bug Fixes
- Support json deserialization and mutation API - (789c874) - Florian Loers
#### Miscellaneous Chores
- Bump harriet version - (79c06b1) - Florian Loers
- Improve parser - (67e5a6d) - Florian Loers

- - -

## v0.4.2 - 2022-07-18
#### Bug Fixes
- **(parser)** Ignore blank nodes instead of failing - (30ca15b) - Florian Loers

- - -

## v0.4.1 - 2022-07-18
#### Bug Fixes
- Allow conditional wasm compilation - (6d4c7ec) - Florian Loers
#### Continuous Integration
- Reintroduce precheck and test steps - (84977f6) - Florian Loers

- - -

## v0.4.0 - 2022-07-17
#### Features
- Add proper WASM bindings - (9903344) - Florian Loers

- - -

## v0.2.0 - 2022-07-12
#### Continuous Integration
- Fix release job versioning - (49a8f48) - Florian Loers
#### Features
- Add wasm capabilities - (9cdfe65) - Florian Loers
- Add turtle parsing with harriet and wasm API - (f462b16) - Florian Loers
#### Miscellaneous Chores
- Rename to owlish - (edbea35) - Florian Loers

- - -

## v0.1.0 - 2022-05-11
#### Features
- Improve owlib convinience and documentation - (f038876) - Florian Loers

- - -

## v0.0.1 - 2022-05-11
#### Miscellaneous Chores
- Setup initial draft version of owl library - (8e105f4) - Florian Loers
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).