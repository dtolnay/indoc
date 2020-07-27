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

mod error;

use crate::error::{Error, Result};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::{self, FromIterator};
use std::str::FromStr;
use unindent::unindent;

#[derive(Copy, Clone, PartialEq)]
enum Macro {
    Indoc,
    Format,
    Print,
}

#[proc_macro]
pub fn indoc(input: TokenStream) -> TokenStream {
    expand(input, Macro::Indoc)
}

#[proc_macro]
pub fn formatdoc(input: TokenStream) -> TokenStream {
    expand(input, Macro::Format)
}

#[proc_macro]
pub fn printdoc(input: TokenStream) -> TokenStream {
    expand(input, Macro::Print)
}

fn expand(input: TokenStream, mode: Macro) -> TokenStream {
    match try_expand(input, mode) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn try_expand(input: TokenStream, mode: Macro) -> Result<TokenStream> {
    let mut input = input.into_iter();
    let first = input.next().ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "unexpected end of macro invocation, expected format string",
        )
    })?;

    let unindented_lit = lit_indoc(first, mode)?;

    if mode == Macro::Indoc {
        match input.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' && input.next().is_none() => {}
            None => {}
            Some(_) => {
                return Err(Error::new(
                    Span::call_site(),
                    &format!(
                        "argument must be a single string literal, but got {} tokens",
                        2 + input.count(),
                    ),
                ));
            }
        }
    }

    let macro_name = match mode {
        Macro::Indoc => return Ok(TokenStream::from(TokenTree::Literal(unindented_lit))),
        Macro::Format => "format",
        Macro::Print => "print",
    };

    // #macro_name! { #unindented_lit #args }
    Ok(TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new(macro_name, Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            iter::once(TokenTree::Literal(unindented_lit))
                .chain(input)
                .collect(),
        )),
    ]))
}

fn lit_indoc(token: TokenTree, mode: Macro) -> Result<Literal> {
    let repr = token.to_string();
    let is_string = repr.starts_with('"') || repr.starts_with('r');
    let is_byte_string = repr.starts_with("b\"") || repr.starts_with("br");

    if !is_string && !is_byte_string {
        return Err(Error::new(
            token.span(),
            "argument must be a single string literal",
        ));
    }

    if is_byte_string && mode != Macro::Indoc {
        return Err(Error::new(
            token.span(),
            "byte strings are not supported in formatting macros",
        ));
    }

    let begin = repr.find('"').unwrap() + 1;
    let end = repr.rfind('"').unwrap();
    let repr = format!(
        "{open}{content}{close}",
        open = &repr[..begin],
        content = unindent(&repr[begin..end]),
        close = &repr[end..],
    );

    match TokenStream::from_str(&repr)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
    {
        TokenTree::Literal(mut lit) => {
            lit.set_span(token.span());
            Ok(lit)
        }
        _ => unreachable!(),
    }
}
