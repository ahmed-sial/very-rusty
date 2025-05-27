fn main(){
    some_string(&Some(String::from("Hello")));
    some_string(&None);
}
fn some_string(x: &Option<String>) {
    match x {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}
