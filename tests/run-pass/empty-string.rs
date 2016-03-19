#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!("");
    let expected = "";
    assert_eq!(indoc, expected);
}
