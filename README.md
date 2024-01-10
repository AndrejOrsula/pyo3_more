# pyo3_more

<p align="left">
  <a href="https://crates.io/crates/pyo3_more">                                   <img alt="crates.io" src="https://img.shields.io/crates/v/pyo3_more.svg"></a>
  <a href="https://github.com/AndrejOrsula/pyo3_more/actions/workflows/rust.yml"> <img alt="Rust"      src="https://github.com/AndrejOrsula/pyo3_more/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://codecov.io/gh/AndrejOrsula/pyo3_more">                         <img alt="codecov"   src="https://codecov.io/gh/AndrejOrsula/pyo3_more/branch/main/graph/badge.svg"></a>
</p>

More macros for [PyO3](https://pyo3.rs).

## Overview

The workspace contains these packages:

- **[pyo3_derive_more](pyo3_derive_more):** More procedural macros for PyO3
- **[pyo3_macros_more](pyo3_macros_more):** More declarative macros for PyO3

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

Add `pyo3_derive_more` and/or `pyo3_macros_more` as Rust dependencies to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest.

```toml
[dependencies]
pyo3_derive_more = "0.1"
pyo3_macros_more = "0.1"
```

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
