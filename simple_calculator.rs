#![allow(warnings)]
use std::{f64::INFINITY, hint, io, result};
fn main() {
    let x = get_input("Enter the first number: ");
    let y = get_input("Enter the second number: ");
    let op = get_operator("Enter the operation (+, -, *, /): ");
    let result = simple_calculator(x, y, op);
    println!("Result: {}", result);
}
fn get_input(prompt: &str) -> f64 {
    let x: f64 = loop {
        let mut input = String::new();
        println!("{}: ", prompt);
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
}fn get_operator(prompt: &str) -> char {
    let x: char = loop {
        let mut input = String::new();
        println!("{}: ", prompt);
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
fn simple_calculator(x: f64, y: f64, op: char) -> f64 {
    match op {
       '+' => x + y,
       '-' => x - y,
       '*' => x * y,
       '/' => x / y,
       _ => f64::INFINITY,
    }
}
