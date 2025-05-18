#![allow(warnings)]
use std::{hint, io};
fn main() {
    let x = get_input();
    let grade = grade_calculator(x);
    println!("Grade: {}", grade);
}
fn get_input() -> u32 {
    let x: u32 = loop {
        let mut input = String::new();
        println!("Enter the your marks: ");
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
fn grade_calculator(x: u32) -> char {
    match x {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        50..=59 => 'E',
        _ => 'F',
    }
}
/*
Rust evaluates match x.
It checks each pattern top to bottom.
When a pattern matches (say, x = 72, matches 70..=79), it evaluates the corresponding expression ('C').
That expression becomes the result of the entire match.
Since match is the last expression in the function, it becomes the return value of the function.
*/
