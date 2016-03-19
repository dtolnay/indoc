#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    indoc!("");
    //~^ argument must start with '\n'
}
