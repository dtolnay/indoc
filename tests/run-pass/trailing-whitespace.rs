#![feature(plugin)]
#![plugin(indoc)]

fn main() {
    let indoc = indoc!("
        2 below
          
        0 below
        
        -2 below
      
        end");
    let expected = "2 below\n  \n0 below\n\n-2 below\n\nend";
    assert_eq!(indoc, expected);
}
