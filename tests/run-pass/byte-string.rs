// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!(b"
        a

            \\b
        c");
    let expected = b"a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}
