extern crate proc_macro;

#[cfg(not(feature = "unstable"))]
use proc_macro_hack::proc_macro_hack;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Lit, LitByteStr, LitStr, Result};
use unindent::*;

#[cfg_attr(feature = "unstable", proc_macro)]
#[cfg_attr(not(feature = "unstable"), proc_macro_hack)]
pub fn indoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let unindented = match try_indoc(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(unindented)
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

    let lit = match syn::parse2::<Lit>(input) {
        Ok(lit) => lit,
        Err(_) => {
            return Err(Error::new(
                Span::call_site(),
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
            let v = unindent_bytes(&lit.value());
            Lit::ByteStr(LitByteStr::new(&v, lit.span()))
        }
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "argument must be a single string literal",
            ));
        }
    };

    Ok(quote!(#lit))
}
