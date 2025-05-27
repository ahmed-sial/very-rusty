fn main(){
    some_number(&Some(4));
    some_number(&None);
}
fn some_number(x: &Option<i32>) {
    match x {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}
