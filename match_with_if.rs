fn main(){
    is_teen(&12);
    is_teen(&20);
}
fn is_teen(x: &u8) {
    match x {
        n if *n <= 13 && *n <= 19 => println!("Teen") ,
        _ => println!("Not a teen"),
    }
}