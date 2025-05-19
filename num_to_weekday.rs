#![allow(warnings)]
use std::{f64::INFINITY, hint, io, result};
fn main() {
    let x = get_input("Enter the number: ");
    let result = get_week_day(x);
    println!("Result: {}", result);
}
fn get_input(prompt: &str) -> u8 {
    let x: u8 = loop {
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
fn get_week_day(n: u8) -> &'static str {
    match n {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day number!",
    }
}
