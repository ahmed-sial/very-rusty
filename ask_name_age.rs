#![allow(warnings)]
use std::{hint, io};
fn main() {
    println!("Enter your name: "); 
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    };
    let mut age = String::new();

    let mut age: u32 = loop {
        println!("Enter your age: ");
        match io::stdin().read_line(&mut age) {
            Ok(_) => {},
            Err(err) => {
                println!("Error: {}", err);
                age.clear();
                continue;
            }
        };
        match age.trim().parse() {
            Ok(num) => break num,
            Err(err) => {
                println!("Error: {}", err);
                age.clear();
                continue;
            }
        } 
    };
    name = name.trim().to_string();
    print!("Hello, {}! You will be {} years old next year.\n", name, age + 1);

}
