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

#[cfg_attr(feature = "unstable", proc_macro)]
#[cfg_attr(not(feature = "unstable"), proc_macro_hack)]
pub fn formatdoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let format = match format_indoc(input, "format") {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(format)
}

#[cfg_attr(feature = "unstable", proc_macro)]
#[cfg_attr(not(feature = "unstable"), proc_macro_hack)]
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
