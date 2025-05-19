#![allow(warnings)]
use std::{hint, io};
fn main() {
    let x = get_input();
    let result = match_character(x);
    println!("{} is: {}", x, result);
}
fn get_input() -> char {
    let x: char = loop {
        let mut input = String::new();
        println!("Enter the char: ");
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
fn match_character(x: char) -> String {
    match x {
        'a' | 'e' | 'i' | 'o' | 'u' | 
        'A' | 'E' | 'I' | 'O' | 'U'    => String::from("Vowel"),
        'a'..='z' | 'A'..='Z' => String::from("Consonant"),
        _ => String::from("Non-alphabet"),
    }
}
/* '|' means match any one of these. We use single pipe in match arms.
*/
