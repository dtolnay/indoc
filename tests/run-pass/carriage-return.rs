#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    // Every line in the string ends with \r\n
    let indoc = indoc!("
        a

            \\b
        c");
    let expected = "a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}
