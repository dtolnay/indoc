Indented Documents (indoc)
==========================

[![Build Status](https://api.travis-ci.org/dtolnay/indoc.svg?branch=master)](https://travis-ci.org/dtolnay/indoc)
[![Latest Version](https://img.shields.io/crates/v/indoc.svg)](https://crates.io/crates/indoc)

This crate provides a Rust compiler plugin for indented string literals. The
`indoc!()` macro takes a multiline string literal and un-indents it so the
leftmost non-space character is in the first column.

Installation
============

Indoc is available on [crates.io](https://crates.io/crates/indoc). Use the
following in `Cargo.toml`:

```toml
[dependencies]
indoc = "^0.1"
```

Release notes are available under [GitHub releases](https://github.com/dtolnay/indoc/releases).

Using Indoc
===========

```rust
#![plugin(indoc)]

fn main() {
    let testing = indoc!("
        a
           b
        c
           d");
    let expected = "a\n   b\nc\n   d";
    assert_eq!(testing, expected);
}
```

Indoc also works with raw string literals:

```rust
#![plugin(indoc)]

fn main() {
    let testing = indoc!(r#"
        a
           "b"
        c
           d
        "#);
    let expected = "a\n   \"b\"\nc\n   d\n";
    assert_eq!(testing, expected);
}
```

And byte string literals:

```rust
#![plugin(indoc)]

fn main() {
    let testing = indoc!(b"
        a
           b
        c
           d
        ");
    let expected = b"a\n   b\nc\n   d\n";
    assert_eq!(testing, expected);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Indoc by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
