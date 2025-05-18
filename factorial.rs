#![allow(warnings)]
use std::{hint, io};
use std::cmp::Ordering;
fn main() {
    let mut num: u32 = loop {
        println!("Enter the number: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => println!("error: {}!", err),
        };
        
        match input.trim().parse() {
           Ok(n) => break n,
           Err(err) => {
               input.clear();
               println!("error: {}!", err);
           },
        };
    };
    println!("Factorial: {}", factorial(&mut num));
}
fn factorial(x: &mut u32) -> u32 {
    if *x > 12 {
        println!("Too large numebr(1..=12)!");
        return 0;
    }
    let mut ans = 1;
    while *x != 1 {
        ans *= *x;
        *x -= 1;
    }
    ans
}
