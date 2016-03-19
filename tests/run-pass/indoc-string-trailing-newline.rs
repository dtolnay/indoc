#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!("
        a

            \\b
        c
    ");
    let expected = "a\n\n    \\b\nc\n";
    assert_eq!(indoc, expected);
}
