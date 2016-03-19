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

use syntax::codemap::Span;
use syntax::parse;
use syntax::parse::token::{self, Lit, Literal, InternedString};
use syntax::ast::{TokenTree, LitKind, StrStyle};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // trait for expr_lit

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

fn expand_indoc<'a>(cx: &'a mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<MacResult + 'a>
{
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument must be a single string literal, but got {} arguments", args.len()));
        return DummyResult::any(sp);
    }

    let lit = match args[0] {
        TokenTree::Token(_, Literal(lit, _name)) => lit,
        _ => {
            cx.span_err(sp, "argument must be a single string literal");
            return DummyResult::any(sp);
        }
    };

    let (input, style) = match lit {
        Lit::Str_(name) =>
            (name.as_str(), StrStyle::Cooked),
        Lit::StrRaw(name, hashes) =>
            (name.as_str(), StrStyle::Raw(hashes)),
        _ => {
            cx.span_err(sp, "argument must be a single string literal");
            return DummyResult::any(sp);
        }
    };

    let unindented = unindent(input);
    let parsed = match style {
        StrStyle::Cooked => parse::str_lit(&unindented),
        StrStyle::Raw(_) => parse::raw_str_lit(&unindented),
    };
    let interned = token::intern_and_get_ident(&parsed);
    let styled = LitKind::Str(interned, style);

    MacEager::expr(cx.expr_lit(sp, styled))
}

// Compute the maximal number of spaces that can be removed from every line, and
// remove them.
fn unindent(input: InternedString) -> String {
    // Document may start either on the same line as opening quote or
    // on the next line
    let ignore_first_line = input.starts_with('\n')
                            || input.starts_with("\r\n");

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
            return Some(i)
        }
    }
    None
}
