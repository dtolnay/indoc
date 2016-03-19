Indented Documents (indoc)
==========================

This Rust crate provides a compiler plugin for indented string literals.

Installation
============

```toml
[dependencies]
indoc = "^0.1"
```

Using Indoc
===========

The `indoc!` macro trims the widest possible rectangle of spaces from the
beginning of lines in its argument:

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

To add a trailing newline:

```rust
#![plugin(indoc)]

fn main() {
    let testing = indoc!("
        a
           b
        c
           d
        ");
    let expected = "a\n   b\nc\n   d\n";
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

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Indoc by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
