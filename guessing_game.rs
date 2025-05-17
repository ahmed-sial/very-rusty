use std::io; 
use rand::{self, Rng};
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("{}", "Welcome to the guessing game!".cyan());
    let mut rng = rand::rng();
    let secret_num = rng.random_range(1..=10);
    loop {
        println!("{}", "Your guess: ".yellow());
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess).expect("Failed to read the input.");
        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        };
    }
}
