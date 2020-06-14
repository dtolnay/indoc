//! [![github]](https://github.com/dtolnay/indoc)&ensp;[![crates-io]](https://crates.io/crates/indoc)&ensp;[![docs-rs]](https://docs.rs/indoc)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This crate provides a procedural macro for indented string literals. The
//! `indoc!()` macro takes a multiline string literal and un-indents it so the
//! leftmost non-space character is in the first column.
//!
//! ```toml
//! [dependencies]
//! indoc = "0.3"
//! ```
//!
//! Release notes are available under [GitHub releases](https://github.com/dtolnay/indoc/releases).
//!
//! # Using Indoc
//!
//! ```
#![cfg_attr(feature = "unstable", doc = " #![feature(proc_macro_hygiene)]")]
#![cfg_attr(feature = "unstable", doc = "")]
//! use indoc::indoc;
//!
//! fn main() {
//!     let testing = indoc!("
//!         def hello():
//!             print('Hello, world!')
//!
//!         hello()
//!         ");
//!     let expected = "def hello():\n    print('Hello, world!')\n\nhello()\n";
//!     assert_eq!(testing, expected);
//! }
//! ```
//!
//! Indoc also works with raw string literals:
//!
//! ```
#![cfg_attr(feature = "unstable", doc = " #![feature(proc_macro_hygiene)]")]
#![cfg_attr(feature = "unstable", doc = "")]
//! use indoc::indoc;
//!
//! fn main() {
//!     let testing = indoc!(r#"
//!         def hello():
//!             print("Hello, world!")
//!
//!         hello()
//!         "#);
//!     let expected = "def hello():\n    print(\"Hello, world!\")\n\nhello()\n";
//!     assert_eq!(testing, expected);
//! }
//! ```
//!
//! And byte string literals:
//!
//! ```
#![cfg_attr(feature = "unstable", doc = " #![feature(proc_macro_hygiene)]")]
#![cfg_attr(feature = "unstable", doc = "")]
//! use indoc::indoc;
//!
//! fn main() {
//!     let testing = indoc!(b"
//!         def hello():
//!             print('Hello, world!')
//!
//!         hello()
//!         ");
//!     let expected = b"def hello():\n    print('Hello, world!')\n\nhello()\n";
//!     assert_eq!(testing[..], expected[..]);
//! }
//! ```
//!
//! # Explanation
//!
//! The following rules characterize the behavior of the `indoc!()` macro:
//!
//! 1. Count the leading spaces of each line, ignoring the first line and any lines
//!    that are empty or contain spaces only.
//! 2. Take the minimum.
//! 3. If the first line is empty i.e. the string begins with a newline, remove the
//!    first line.
//! 4. Remove the computed number of spaces from the beginning of each line.
//!
//! This means there are a few equivalent ways to format the same string, so choose
//! one you like. All of the following result in the string `"line one\nline
//! two\n"`:
//!
//! ```text
//! indoc!("            /      indoc!(             /      indoc!("line one
//!    line one        /         "line one        /               line two
//!    line two       /           line two       /                ")
//!    ")            /            ")            /
//! ```

#![cfg_attr(feature = "unstable", feature(decl_macro))]
#![cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#![no_std]

#[cfg(not(feature = "unstable"))]
use proc_macro_hack::proc_macro_hack;

#[cfg_attr(not(feature = "unstable"), proc_macro_hack)]
pub use indoc_impl::indoc;
