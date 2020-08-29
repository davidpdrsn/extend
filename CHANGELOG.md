# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

- Handle unnamed extensions on the same generic type with different type parameters. For example `Option<i32>` and `Option<String>`. Previously we would generate the same name of both hidden traits which wouldn't compile.

### Breaking changes

- Generated traits are no longer sealed and the `sealed` argument previously supported by `#[ext]` has been removed. Making the traits sealed lead to lots of complexity that we didn't think brought much value.

## 0.1.1 - 2020-02-22

- Add support for specifying supertraits of the generated trait [#4](https://github.com/davidpdrsn/extend/pull/4).

## 0.1.0

- Support adding extensions to the ["never type"](https://doc.rust-lang.org/std/primitive.never.html).

### Breaking changes

- Simplify names of traits generates for complex types.

## 0.0.2

- Move "trybuild" to dev-dependency.

## 0.0.1

Initial release.
