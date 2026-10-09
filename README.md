Indented Documents (indoc)
==========================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/indoc-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/indoc)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indoc.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/indoc)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-indoc-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/indoc)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/indoc/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/indoc/actions?query=branch%3Amaster)

This crate provides a procedural macro for indented string literals. The
`indoc!()` macro takes a multiline string literal and un-indents it at compile
time so the leftmost non-space character is in the first column.

```toml
[dependencies]
indoc = "2"
```

<br>

## Using indoc

```rust
use indoc::indoc;

fn main() {
    let testing = indoc! {"
        def hello():
            print('Hello, world!')

        hello()
    "};
    let expected = "def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

Indoc also works with raw string literals:

```rust
use indoc::indoc;

fn main() {
    let testing = indoc! {r#"
        def hello():
            print("Hello, world!")

        hello()
    "#};
    let expected = "def hello():\n    print(\"Hello, world!\")\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

And byte string literals:

```rust
use indoc::indoc;

fn main() {
    let testing = indoc! {b"
        def hello():
            print('Hello, world!')

        hello()
    "};
    let expected = b"def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing[..], expected[..]);
}
```

<br>

## Formatting macros

The indoc crate exports six additional macros to substitute conveniently for
the standard library's formatting macros:

- `formatdoc!($fmt, ...)` — unindent the format string and call [`format!`][std-format].
- `printdoc!($fmt, ...)` — unindent the format string and call [`print!`][std-print].
- `eprintdoc!($fmt, ...)` — unindent the format string and call [`eprint!`][std-eprint].
- `writedoc!($dest, $fmt, ...)` — unindent the format string and call [`write!`][std-write].
- `concatdoc!(...)` — unindent each string literal and call [`concat!`][std-concat].
- `panicdoc!($fmt, ...)` — unindent the format string and call [`panic!`][std-panic].

[std-format]: https://doc.rust-lang.org/std/macro.format.html
[std-print]: https://doc.rust-lang.org/std/macro.print.html
[std-eprint]: https://doc.rust-lang.org/std/macro.eprint.html
[std-write]: https://doc.rust-lang.org/std/macro.write.html
[std-concat]: https://doc.rust-lang.org/std/macro.concat.html
[std-panic]: https://doc.rust-lang.org/std/macro.panic.html

```rust
use indoc::{concatdoc, printdoc};

const HELP: &str = concatdoc! {"
    Usage: ", env!("CARGO_BIN_NAME"), " [options]

    Options:
        -h, --help
"};

fn main() {
    printdoc! {"
        GET {url}
        Accept: {mime}
        ",
        url = "http://localhost:8080",
        mime = "application/json",
    }
}
```

### Variable capture

Rust deliberately disables implicit variable capture in macro-generated format strings to avoid
ambiguity about variable scope ([RFC 2795][format-capture-hygiene]). To unindent and format a
string, pass formatting arguments explicitly when nesting `indoc!`, or use `formatdoc!` or
`printdoc!` with implicit capture. Both `formatdoc!` and `printdoc!` require a string literal
as their format string; they do not accept another macro invocation:

```rust
use indoc::{formatdoc, indoc};

let name = "world";
assert_eq!(formatdoc!("Hello {name}"), "Hello world");
assert_eq!(format!(indoc!("Hello {name}"), name = name), "Hello world");
```

[format-capture-hygiene]:
  https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html#macro-hygiene

<br>

## Explanation

The following rules characterize the behavior of the `indoc!()` macro:

1. Count the leading spaces of each line, ignoring the first line and any lines
   that are empty or contain spaces only.
2. Take the minimum.
3. If the first line is empty i.e. the string begins with a newline, remove the
   first line.
4. Remove the computed number of spaces from the beginning of each line.

<br>

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

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
