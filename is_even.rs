#![allow(warnings)]
use std::{hint, io};
fn main() {
    let x: i32 = loop {
        let mut input = String::new();
        println!("Enter the number: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(err) => {
                input.clear();
                println!("error: {}", err);
            },
        }
        match input.trim().parse() {
            Ok(n) => break n,
            Err(err) => {
                input.clear();
                println!("error: {}", err);
            }
        }
    };
    let is_even = is_even(x);
    if is_even == true {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
}
fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    false
}
