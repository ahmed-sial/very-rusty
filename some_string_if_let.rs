fn main(){
    some_string(&Some(String::from("Hello")));
    some_string(&None);
}
fn some_string(x: &Option<String>) {
    if let Some(x) = x {
        println!("{}", x);
    } else {
        println!("None");
    }
}
