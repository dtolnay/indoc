//! [![github]](https://github.com/dtolnay/indoc)&ensp;[![crates-io]](https://crates.io/crates/unindent)&ensp;[![docs-rs]](https://docs.rs/unindent)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! ## Unindent
//!
//! This crate provides [`indoc`]'s indentation logic for use with strings that
//! are not statically known at compile time. For unindenting string literals,
//! use `indoc` instead.
//!
//! [`indoc`]: https://github.com/dtolnay/indoc
//!
//! This crate exposes two unindent functions and an extension trait:
//!
//! - `fn unindent(&str) -> String`
//! - `fn unindent_bytes(&[u8]) -> Vec<u8>`
//! - `trait Unindent`
//!
//! ```
//! use unindent::unindent;
//!
//! fn main() {
//!     let indented = "
//!             line one
//!             line two";
//!     assert_eq!("line one\nline two", unindent(indented));
//! }
//! ```
//!
//! The `Unindent` extension trait expose the same functionality under an
//! extension method.
//!
//! ```
//! use unindent::Unindent;
//!
//! fn main() {
//!     let indented = format!("
//!             line {}
//!             line {}", "one", "two");
//!     assert_eq!("line one\nline two", indented.unindent());
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/unindent/0.1.9")]
#![allow(
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_complexity
)]

mod unindent;

pub use unindent::*;
