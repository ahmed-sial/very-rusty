struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn which_axis(&self) -> () {
        if self.x == 0 && self.y == 0 {
            println!("origin");
        } else if self.x == 0 {
            println!("y-axis");
        } else if self.y == 0 {
            println!("y-axis");
        } else {
            println!("x: {}, y: {}", self.x, self.y);
        }
        ()
    }
}
fn main(){
    let p = Point {
        x: 2,
        y: 0,
    };
    p.which_axis();
}
