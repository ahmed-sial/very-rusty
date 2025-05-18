#![allow(warnings)]
use std::{hint, io};
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Sum: {}", sum(arr));
    println!("Product: {}", multiply(arr));
    println!("Average: {}", average(arr));

}
fn sum(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    loop {
        sum += arr[i]; 
        if (i == 4) {
            break;
        }
        i += 1;
    }
    sum
}
fn multiply(arr: [i32; 5]) -> i32 {
    let mut product = 1;
    let mut i = 0;
    loop {
        product *= arr[i]; 
        if (i == 4) {
            break;
        }
        i += 1;
    }
   product 
}
fn average(arr: [i32; 5]) -> f64 {
    let mut sum = 0;
    let mut i = 0;
    loop {
        sum += arr[i]; 
        if (i == 4) {
            break;
        }
        i += 1;
    }
    //(sum as f64 / 5 as f64) also correct
    (sum as f64 / 5.0)
}
