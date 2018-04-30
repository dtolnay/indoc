// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ## Unindent
//!
//! This crate provides [`indoc`]'s indentation logic for use with strings that
//! are not statically known at compile time. For unindenting string literals,
//! use `indoc` instead.
//!
//! [`indoc`]: https://github.com/dtolnay/indoc
//!
//! This crate exposes two functions:
//!
//! - `unindent(&str) -> String`
//! - `unindent_bytes(&[u8]) -> Vec<u8>`
//!
//! ```rust
//! extern crate unindent;
//! use unindent::unindent;
//!
//! fn main() {
//!     let indented = "
//!             line one
//!             line two";
//!     assert_eq!("line one\nline two", unindent(indented));
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/unindent/0.1.3")]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]

use std::iter::Peekable;
use std::slice::Split;

pub fn unindent(s: &str) -> String {
    let bytes = s.as_bytes();
    let unindented = unindent_bytes(bytes);
    String::from_utf8(unindented).unwrap()
}

// Compute the maximal number of spaces that can be removed from every line, and
// remove them.
pub fn unindent_bytes(s: &[u8]) -> Vec<u8> {
    // Document may start either on the same line as opening quote or
    // on the next line
    let ignore_first_line = s.starts_with(b"\n") || s.starts_with(b"\r\n");

    // Largest number of spaces that can be removed from every
    // non-whitespace-only line after the first
    let spaces = s.lines()
        .skip(1)
        .filter_map(count_spaces)
        .min()
        .unwrap_or(0);

    let mut result = Vec::with_capacity(s.len());
    for (i, line) in s.lines().enumerate() {
        if i > 1 || (i == 1 && !ignore_first_line) {
            result.push(b'\n');
        }
        if i == 0 {
            // Do not un-indent anything on same line as opening quote
            result.extend_from_slice(line);
        } else if line.len() > spaces {
            // Whitespace-only lines may have fewer than the number of spaces
            // being removed
            result.extend_from_slice(&line[spaces..]);
        }
    }
    result
}

// Number of leading spaces in the line, or None if the line is entirely spaces.
fn count_spaces(line: &[u8]) -> Option<usize> {
    for (i, ch) in line.iter().enumerate() {
        if *ch != b' ' {
            return Some(i);
        }
    }
    None
}

// Based on core::str::StrExt.
trait BytesExt {
    fn lines(&self) -> Lines;
}

impl BytesExt for [u8] {
    fn lines(&self) -> Lines {
        fn is_newline(b: &u8) -> bool {
            *b == b'\n'
        }
        Lines {
            split: self.split(is_newline as fn(&u8) -> bool).peekable(),
        }
    }
}

struct Lines<'a> {
    split: Peekable<Split<'a, u8, fn(&u8) -> bool>>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.split.next() {
            None => None,
            Some(fragment) => {
                if fragment.is_empty() && self.split.peek().is_none() {
                    None
                } else {
                    Some(fragment)
                }
            }
        }
    }
}
