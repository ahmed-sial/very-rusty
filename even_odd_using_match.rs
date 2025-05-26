fn main() {
    println!("{}", check_even_odd_using_match(12));
}
fn check_even_odd_using_match(x: i32) -> &'static str {
    match x % 2 {
        0 => "Even",
        _ => "Odd",
    }
}
