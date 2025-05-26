fn main() {
    println!("{}", match_pattern(&12));
}
fn match_pattern<'a>(x: &'a i32) -> &'a str {
    match x {
       0 => "Zero",
       1 => "One",
       _ => "Many",
    }
}
