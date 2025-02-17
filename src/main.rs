#![allow(dead_code)]



mod bad_stack;
use bad_stack::*;




fn main() {
    println!("Hello, world!");
    
    let mut a: List = List::new();
    a.push(3);
    a.push(5);
    hello();

    println!("{:#?}", a);

}
