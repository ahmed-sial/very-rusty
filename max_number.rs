#![allow(warnings)]
use std::{hint, io};
fn main() {
    let x = get_input(); 
    let y = get_input();
    let max = max(x, y);
    println!("Max between {} and {} is {}", x, y, max);
}
fn get_input() -> i32 {
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
    x
}
fn max(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    }
    y
}
