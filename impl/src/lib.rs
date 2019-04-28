// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proc_macro;

#[cfg(not(feature = "unstable"))]
use proc_macro_hack::proc_macro_hack;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Lit, LitByteStr, LitStr};
use unindent::*;

#[cfg_attr(feature = "unstable", proc_macro)]
#[cfg_attr(not(feature = "unstable"), proc_macro_hack)]
pub fn indoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = TokenStream::from(input);

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

    proc_macro::TokenStream::from(quote!(#lit))
}
