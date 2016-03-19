#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    indoc!("
        a
        b
        c
        " 64);
    //~^^^^^ argument must be a single string literal, but got 2 arguments
}
