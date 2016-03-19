#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!(r#"
        "a"

            \\b
        c"#);
    let expected = "\"a\"\n\n    \\\\b\nc";
    assert_eq!(indoc, expected);
}
