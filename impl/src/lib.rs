// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/indoc-impl/0.2.6")]
#![cfg_attr(feature = "unstable", feature(proc_macro))]

#[cfg(feature = "unstable")]
extern crate proc_macro;

#[cfg(not(feature = "unstable"))]
#[macro_use]
extern crate proc_macro_hack;

extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

extern crate unindent;
use unindent::*;

use proc_macro2::TokenStream;
use syn::{Lit, LitByteStr, LitStr};

use std::fmt::Debug;
use std::str::FromStr;

#[cfg(feature = "unstable")]
#[proc_macro]
pub fn indoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(&input)
}

#[cfg(not(feature = "unstable"))]
proc_macro_expr_impl! {
    pub fn indoc_impl(input: &str) -> String {
        expand(input)
    }
}

fn expand<T, R>(input: &T) -> R
where
    T: ?Sized + ToString,
    R: FromStr,
    R::Err: Debug,
{
    let source = input.to_string().parse::<TokenStream>().unwrap();

    let len = source.clone().into_iter().count();
    if len != 1 {
        panic!(
            "argument must be a single string literal, but got {} tokens",
            len
        );
    }

    let lit = match syn::parse2::<Lit>(source) {
        Ok(lit) => lit,
        Err(_) => {
            panic!("argument must be a single string literal");
        }
    };

    let lit = match lit {
        Lit::Str(lit) => {
            let v = unindent(&lit.value());
            Lit::Str(LitStr::new(&v, lit.span()))
        }
        Lit::ByteStr(lit) => {
            let v = unindent_bytes(&lit.value());
            Lit::ByteStr(LitByteStr::new(&v, lit.span()))
        }
        _ => {
            panic!("argument must be a single string literal");
        }
    };

    quote!(#lit).to_string().parse().unwrap()
}
