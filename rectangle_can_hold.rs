#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}
impl Rectangle {
    // Just extra function, not used in program
    fn equals(&self, other: &Rectangle) -> bool {
        self.height == other.height && self.width == other.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}
fn main() {
    let r1 = Rectangle {
        height: 20,
        width: 10,
    };
    let r2 = Rectangle {
        height: 10,
        width: 10,
    };
    println!("{}", r1.can_hold(&r2));
}
