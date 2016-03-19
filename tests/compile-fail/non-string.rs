#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    indoc!(64);
    //~^ argument must be a single string literal
}
