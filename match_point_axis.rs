struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn which_axis(&self) -> () {
       match (self.x, self.y) {
          (0, 0) => println!("origin"),
          (0, _) => println!("y-axis"),
          (_, 0) => println!("x-axis"),
          (x, y) => println!("x: {}, y: {}", x, y),
       } 
    }
}
fn main(){
    let p = Point {
        x: 2,
        y: 0,
    };
    p.which_axis();
}
