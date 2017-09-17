// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides a procedural macro for indented string literals. The
//! `indoc!()` macro takes a multiline string literal and un-indents it so the
//! leftmost non-space character is in the first column.
//!
//! ```toml
//! [dependencies]
//! indoc = "0.2"
//! ```
//!
//! Release notes are available under [GitHub releases](https://github.com/dtolnay/indoc/releases).
//!
//! # Using Indoc
//!
//! ```rust
//! #[macro_use]
//! extern crate indoc;
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
//! ```rust
//! #[macro_use]
//! extern crate indoc;
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
//! ```rust
//! #[macro_use]
//! extern crate indoc;
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

#![doc(html_root_url = "https://docs.rs/indoc/0.2.0")]

#![cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate indoc_impl;
#[doc(hidden)]
pub use indoc_impl::*;

proc_macro_expr_decl! {
    indoc! => indoc_impl
}
