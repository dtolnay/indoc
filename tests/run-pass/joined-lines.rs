#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!("
        a\
        b
        c\
          d
        e");
    let expected = "ab\ncd\ne";
    assert_eq!(indoc, expected);
}
