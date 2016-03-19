#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    indoc!("a
            b
            c");
    //~^^^ argument must start with '\n'
}
