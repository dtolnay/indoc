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
//! `indoc` also exports two `format`-like macros - `formatdoc`, which work exactly
//! like `format` and generates the unindented formatted string, and `printdoc`,
//! which prints the unindented formatted string to the standard output:
//!
//! ```
//! use indoc::formatdoc;
//!
//! fn main() {
//!     let testing = formatdoc!("
//!         {}\
//!         {}
//!         {}\
//!           {}
//!         {}", 'a', 'b', 'c', 'd', 'e');
//!     let expected = "ab\ncd\ne";
//!     assert_eq!(testing, expected);
//! }
//! ```
//! Note that these macros, just like `format` and `print`, do not support binary
//! strings. Also, the format string is unindented and not the formatted one:
//! ```
//! use indoc::formatdoc;
//!
//! fn main() {
//!     let testing = formatdoc!("\
//!         {}", " a");
//!     let expected = " a";
//!     // The leading space in the substitution is preserved.
//!     assert_eq!(testing, expected);
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

#![allow(clippy::needless_doctest_main)]

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Lit, LitByteStr, LitStr, Result};
use unindent::*;

#[proc_macro]
pub fn indoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let unindented = match try_indoc(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(unindented)
}

#[proc_macro]
pub fn formatdoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let format = match format_indoc(input, "format") {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(format)
}

#[proc_macro]
pub fn printdoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let format = match format_indoc(input, "print") {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(format)
}

fn try_indoc(input: TokenStream) -> Result<TokenStream> {
    let len = input.clone().into_iter().count();
    if len != 1 {
        return Err(Error::new(
            Span::call_site(),
            format!(
                "argument must be a single string literal, but got {} tokens",
                len,
            ),
        ));
    }
    lit_indoc(input, true)
}

fn lit_indoc(input: TokenStream, raw: bool) -> Result<TokenStream> {
    let lit = match syn::parse2::<Lit>(input) {
        Ok(lit) => lit,
        Err(err) => {
            return Err(Error::new(
                err.span(),
                "argument must be a single string literal",
            ));
        }
    };

    let lit = match lit {
        Lit::Str(lit) => {
            let v = unindent(&lit.value());
            Lit::Str(LitStr::new(&v, lit.span()))
        }
        Lit::ByteStr(lit) => {
            if raw {
                let v = unindent_bytes(&lit.value());
                Lit::ByteStr(LitByteStr::new(&v, lit.span()))
            } else {
                return Err(Error::new_spanned(
                    lit,
                    "byte strings are not supported in formatting macros",
                ));
            }
        }
        otherwise => {
            return Err(Error::new_spanned(
                otherwise,
                "argument must be a single string literal",
            ));
        }
    };

    Ok(quote!(#lit))
}

fn format_indoc(input: TokenStream, macro_name: &str) -> Result<TokenStream> {
    let macro_name = syn::Ident::new(macro_name, Span::call_site());

    let mut input = input.into_iter();

    let first = std::iter::once(input.next().ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "unexpected end of macro invocation, expected format string",
        )
    })?)
    .collect();
    let fmt_str = lit_indoc(first, false)?;

    let args: TokenStream = input.collect();

    Ok(quote!(#macro_name!(#fmt_str #args)))
}
