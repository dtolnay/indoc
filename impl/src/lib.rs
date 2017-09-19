// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/indoc-impl/0.2.0")]

#![cfg_attr(feature = "unstable", feature(proc_macro))]

#[cfg(feature = "unstable")]
extern crate proc_macro;
#[cfg(feature = "unstable")]
use proc_macro::TokenStream;

#[cfg(not(feature = "unstable"))]
#[macro_use]
extern crate proc_macro_hack;

extern crate syn;

#[macro_use]
extern crate quote;

extern crate unindent;
use unindent::*;

use syn::{TokenTree, Token, Lit};

use std::fmt::Debug;
use std::str::FromStr;

#[cfg(feature = "unstable")]
#[proc_macro]
pub fn indoc(input: TokenStream) -> TokenStream {
    expand(&input)
}

#[cfg(not(feature = "unstable"))]
proc_macro_expr_impl! {
    pub fn indoc_impl(input: &str) -> String {
        expand(input)
    }
}

fn expand<T, R>(input: &T) -> R
    where T: ?Sized + ToString,
          R: FromStr,
          R::Err: Debug
{
    let source = input.to_string();

    let tts = syn::parse_token_trees(&source).unwrap();

    if tts.len() != 1 {
        panic!("argument must be a single string literal, but got {} arguments", tts.len());
    }

    let tt = tts.into_iter().next().unwrap();

    let mut lit = match tt {
        TokenTree::Token(Token::Literal(lit)) => lit,
        _ => {
            panic!("argument must be a single string literal");
        }
    };

    match lit {
        Lit::Str(ref mut s, _style) => {
            *s = unindent(s);
        }
        Lit::ByteStr(ref mut v, _style) => {
            *v = unindent_bytes(v);
        }
        _ => {
            panic!("argument must be a single string literal");
        }
    }

    quote!(#lit).parse().unwrap()
}
