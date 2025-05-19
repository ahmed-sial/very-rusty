#![allow(warnings)]
use std::{f64::INFINITY, hint, io, result};
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    reverse_array(&mut arr);
    for i in arr.iter() {
        println!("{}", *i);
    }
}
fn reverse_array(arr: &mut [i32]) {
    for i in 0..=arr.len() / 2 {
        let temp = arr[i];
        arr[i] = arr[arr.len() - i - 1];
        arr[arr.len() - i - 1] = temp;
    }
}
