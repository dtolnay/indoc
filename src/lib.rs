// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/indoc/0.2.0")]

#![cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate indoc_impl;
#[doc(hidden)]
pub use indoc_impl::*;

proc_macro_expr_decl! {
    indoc! => indoc_impl
}
