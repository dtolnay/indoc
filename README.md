Indented Documents (indoc)
==========================

[![Build Status](https://api.travis-ci.org/dtolnay/indoc.svg?branch=master)](https://travis-ci.org/dtolnay/indoc)
[![Latest Version](https://img.shields.io/crates/v/indoc.svg)](https://crates.io/crates/indoc)

This crate provides a procedural macro for indented string literals. The
`indoc!()` macro takes a multiline string literal and un-indents it so the
leftmost non-space character is in the first column.

```toml
[dependencies]
indoc = "0.3"
```

Release notes are available under [GitHub releases](https://github.com/dtolnay/indoc/releases).

## Using Indoc

```rust
use indoc::indoc;

fn main() {
    let testing = indoc!("
        def hello():
            print('Hello, world!')

        hello()
        ");
    let expected = "def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

Indoc also works with raw string literals:

```rust
use indoc::indoc;

fn main() {
    let testing = indoc!(r#"
        def hello():
            print("Hello, world!")

        hello()
        "#);
    let expected = "def hello():\n    print(\"Hello, world!\")\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

And byte string literals:

```rust
use indoc::indoc;

fn main() {
    let testing = indoc!(b"
        def hello():
            print('Hello, world!')

        hello()
        ");
    let expected = b"def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing[..], expected[..]);
}
```

## Explanation

The following rules characterize the behavior of the `indoc!()` macro:

1. Count the leading spaces of each line, ignoring the first line and any lines
   that are empty or contain spaces only.
2. Take the minimum.
3. If the first line is empty i.e. the string begins with a newline, remove the
   first line.
4. Remove the computed number of spaces from the beginning of each line.

This means there are a few equivalent ways to format the same string, so choose
one you like. All of the following result in the string `"line one\nline
two\n"`:

```
indoc!("            /      indoc!(             /      indoc!("line one
   line one        /         "line one        /               line two
   line two       /           line two       /                ")
   ")            /            ")            /
```

## Unindent

Indoc's indentation logic is available in the `unindent` crate. This may be
useful for processing strings that are not statically known at compile time.

The crate exposes two functions:

- `unindent(&str) -> String`
- `unindent_bytes(&[u8]) -> Vec<u8>`

```rust
use unindent::unindent;

fn main() {
    let indented = "
            line one
            line two";
    assert_eq!("line one\nline two", unindent(indented));
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
