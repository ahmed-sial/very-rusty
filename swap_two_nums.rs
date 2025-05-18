// simple program demonstrating swapping of two numbers;
#![allow(warnings)]
use std::{hint, io};
fn main() {
    let mut x = 10;
    let mut y = 20;
    println!("Before");
    println!("X: {}", x);
    println!("Y: {}", y);
    swap(&mut x, &mut y);
    println!("After");
    println!("X: {}", x);
    println!("Y: {}", y);
}
fn swap(x: &mut i32, y: &mut i32) -> () {
    let mut temp = *x;
    *x = *y;
    *y = temp;

}
