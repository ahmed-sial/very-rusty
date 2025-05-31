use std::io::{ self };

// Question: Write a program that takes two integer values as input from the user. Then swap the
// values taken from the user and display the output of the variables. (Value of num1 should
// be stored in num2 and vice versa). DO not use any other variable for swapping.

fn main() {
    let mut num1 = take_input("Enter the first number: ");
    let mut num2 = take_input("Enter the second number: ");
    println!("Before -> x: {}, y: {}", num1, num2);
    swap_without_other_variable(&mut num1, &mut num2);
    println!("After -> x: {}, y: {}", num1, num2);
}

fn take_input(str: &str) -> i32 {
    println!("{}", str);
    let x: i32 = loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                println!("error: {}", err);
                input.clear();
                continue;
            }
        }
        match input.trim().parse() {
            Ok(n) => {
                break n;
            }
            Err(err) => {
                println!("error: {}", err);
                input.clear();
                continue;
            }
        }
    };
    x
}

fn swap_without_other_variable(x: &mut i32, y: &mut i32) {
    *x += *y;
    *y = *x - *y;
    *x -= *y;
}
