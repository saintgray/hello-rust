use std::io::{stdin, Read};
use va

fn main() {
    print_nav();
    let mut input = String::new();
    stdin().read_to_string( &mut input).unwrap();
    
    let test: &str = &String::from(&input);
    let test2: &str = &String::from("1");
    println!("{:?}",test.as_bytes());
    println!("{:?}",test2.as_bytes());
    if String::eq(&input.to_string(), &"1") {
        println!("selected variable");
    } else {
        println!("selected {input}");
    }
}


fn print_nav() {
    println!("----------------------");
    println!("1. variable");
    println!("2. collection");
    println!("3. exit               ");
    println!("----------------------");
}
 