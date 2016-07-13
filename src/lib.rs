// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))] // turn warnings into errors

#![cfg_attr(not(feature = "with-syntex"), feature(plugin_registrar, rustc_private))]

#[cfg(not(feature = "with-syntex"))]
extern crate rustc_plugin;
#[cfg(not(feature = "with-syntex"))]
extern crate syntax;

#[cfg(feature = "with-syntex")]
extern crate syntex;
#[cfg(feature = "with-syntex")]
extern crate syntex_syntax as syntax;

#[cfg(feature = "with-syntex")]
use std::path::Path;

use syntax::codemap::Span;
use syntax::parse;
use syntax::parse::token::{self, Lit, Literal};
use syntax::ast::{LitKind, Name, StrStyle};
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::ext::build::AstBuilder; // trait for expr_lit
use syntax::tokenstream::TokenTree;

#[cfg(not(feature = "with-syntex"))]
#[plugin_registrar]
#[doc(hidden)]
pub fn register(reg: &mut rustc_plugin::Registry) {
    reg.register_macro("indoc", expand_indoc);
}

#[cfg(feature = "with-syntex")]
#[doc(hidden)]
pub fn register(reg: &mut syntex::Registry) {
    reg.add_macro("indoc", expand_indoc);
}

#[cfg(feature = "with-syntex")]
#[doc(hidden)]
pub fn expand<S, D>(src: S, dst: D) -> Result<(), syntex::Error>
    where S: AsRef<Path>,
          D: AsRef<Path>,
{
    let mut registry = syntex::Registry::new();
    register(&mut registry);
    registry.expand("", src.as_ref(), dst.as_ref())
}

fn expand_indoc<'a>(
    cx: &'a mut ExtCtxt,
    sp: Span,
    args: &[TokenTree]
) -> Box<MacResult + 'a> {
    if args.len() != 1 {
        cx.span_err(sp,
                    &format!("argument must be a single string literal, but \
                              got {} arguments",
                             args.len()));
        return DummyResult::any(sp);
    }

    let lit = match args[0] {
        TokenTree::Token(_, Literal(lit, _name)) => lit,
        _ => {
            cx.span_err(sp, "argument must be a single string literal");
            return DummyResult::any(sp);
        }
    };

    MacEager::expr(cx.expr_lit(sp,
                               match lit {
                                   Lit::Str_(name) =>
            LitKind::Str(
                token::intern_and_get_ident(
                    &parse::str_lit(&unindent(name))),
                StrStyle::Cooked),
                                   Lit::StrRaw(name, hashes) =>
            LitKind::Str(
                token::intern_and_get_ident(
                    &parse::raw_str_lit(&unindent(name))),
                StrStyle::Raw(hashes)),
                                   Lit::ByteStr(name) =>
            LitKind::ByteStr(
                parse::byte_str_lit(&unindent(name))),
                                   Lit::ByteStrRaw(name, _hashes) =>
            LitKind::ByteStr(
                parse::byte_str_lit(&unindent(name))),
                                   _ => {
                                       cx.span_err(sp,
                                                   "argument must be a \
                                                    single string literal");
                                       return DummyResult::any(sp);
                                   }
                               }))
}

// Compute the maximal number of spaces that can be removed from every line, and
// remove them.
fn unindent(name: Name) -> String {
    let input = name.as_str();

    // Document may start either on the same line as opening quote or
    // on the next line
    let ignore_first_line = input.starts_with('\n') ||
                            input.starts_with("\r\n");

    // Largest number of spaces that can be removed from every
    // non-whitespace-only line after the first
    let spaces = input.lines()
        .skip(1)
        .filter_map(count_spaces)
        .min()
        .unwrap_or(0);

    let mut result = String::with_capacity(input.len());
    for (i, line) in input.lines().enumerate() {
        if i > 1 || (i == 1 && !ignore_first_line) {
            result.push_str("\n");
        }
        if i == 0 {
            // Do not un-indent anything on same line as opening quote
            result.push_str(line);
        } else if line.len() > spaces {
            // Whitespace-only lines may have fewer than the number of spaces
            // being removed
            result.push_str(&line[spaces..]);
        }
    }
    result
}

// Number of leading spaces in the line, or None if the line is entirely spaces.
fn count_spaces(line: &str) -> Option<usize> {
    for (i, ch) in line.chars().enumerate() {
        if ch != ' ' {
            return Some(i);
        }
    }
    None
}
