#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::codemap::Span;
use syntax::parse::{self, token};
use syntax::ast::{TokenTree, LitKind, StrStyle};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // trait for expr_str
use rustc_plugin::Registry;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("indoc", expand_indoc);
}

fn expand_indoc(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<MacResult + 'static>
{
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument must be a single string literal, but got {} arguments", args.len()));
        return DummyResult::any(sp);
    }

    let lit = match args[0] {
        TokenTree::Token(_, token::Literal(lit, _name)) => lit,
        _ => {
            cx.span_err(sp, "argument must be a single string literal");
            return DummyResult::any(sp);
        }
    };

    let (input, style) = match lit {
        token::Lit::Str_(name) =>
            (name.as_str(), StrStyle::Cooked),
        token::Lit::StrRaw(name, hashes) =>
            (name.as_str(), StrStyle::Raw(hashes)),
        _ => {
            cx.span_err(sp, "argument must be a single string literal");
            return DummyResult::any(sp);
        }
    };

    if !input.starts_with("\n") {
        cx.span_err(sp, "argument must start with '\n'");
        return DummyResult::any(sp);
    }

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
fn unindent(input: token::InternedString) -> String {
    let spaces = input.lines().filter_map(count_spaces).min().unwrap_or(0);

    let mut result = String::new();
    for (i, line) in input[1..].lines().enumerate() {
        if i > 0 {
            result.push_str("\n");
        }
        if let Some(_) = count_spaces(line) {
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
