// simple program to calculate area of rectangle.
#![allow(warnings)]
use std::{hint, io};
fn main() {
    let mut width_str = String::new();
    let mut height_str = String::new();
    let mut width: u32 = loop {
        println!("Enter width: ");
        io::stdin()
            .read_line(&mut width_str)
            .expect("Error occured while reading width.");

        match width_str.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid data!");
                width_str.clear();
                continue;
            },
        };
    };
    let mut height: u32 = loop {
        println!("Enter height: ");
        io::stdin().read_line(&mut height_str)
            .expect("Error occured while reading height.");

        match height_str.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid data!");
                height_str.clear();
                continue;
            },
        };
    };

    let area = calculate_area(width, height);
    println!("The area of rectangle is: {}", area);

}
fn calculate_area(x: u32, y: u32) -> u32 {
    x * y
}
/*
fn print_type_of<T>(_: &T) {
   println!("Type: {}", std::any::type_name::<T>());
*/
