#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!("a
                        b
                        c");
    let expected = "a\nb\nc";
    assert_eq!(indoc, expected);
}
