#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    indoc!();
    //~^ argument must be a single string literal, but got 0 arguments
}
