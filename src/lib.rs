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

extern crate unindent;
use unindent::*;

#[cfg(feature = "with-syntex")]
use std::path::Path;

use syntax::codemap::Span;
use syntax::parse;
use syntax::parse::token::{self, Lit, Literal};
use syntax::ast::{LitKind, StrStyle};
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
                    &parse::str_lit(&unindent(&name.as_str()))),
                StrStyle::Cooked),
                                   Lit::StrRaw(name, hashes) =>
            LitKind::Str(
                token::intern_and_get_ident(
                    &parse::raw_str_lit(&unindent(&name.as_str()))),
                StrStyle::Raw(hashes)),
                                   Lit::ByteStr(name) =>
            LitKind::ByteStr(
                parse::byte_str_lit(&unindent(&name.as_str()))),
                                   Lit::ByteStrRaw(name, _hashes) =>
            LitKind::ByteStr(
                parse::byte_str_lit(&unindent(&name.as_str()))),
                                   _ => {
                                       cx.span_err(sp,
                                                   "argument must be a \
                                                    single string literal");
                                       return DummyResult::any(sp);
                                   }
                               }))
}
