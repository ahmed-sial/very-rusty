fn main(){
    is_teen(&12);
    is_teen(&20);
}
fn is_teen(x: &u8) {
    if (13..=19).contains(x) {
        println!("Teen");
    } else {
        println!("Not a Teen");
    }
}