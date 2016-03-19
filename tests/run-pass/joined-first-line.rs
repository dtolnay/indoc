#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!("\
        a");
    let expected = "a";
    assert_eq!(indoc, expected);
}
