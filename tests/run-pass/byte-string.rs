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
