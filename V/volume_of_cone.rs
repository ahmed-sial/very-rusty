use std::f64::consts;
// Question: Calculate the volume of a given cone, using float data type for all values, and output the
// final answer up to 3 decimal places exactly.
fn main() {
    let r = get_input("Enter the radius: ");
    let h = get_input("Enter the height: ");
    if r == 0.0 || h == 0.0{
        println!("Volume: 0.0");
    } else if h < 0.0 {
        panic!("error: negative height.");
    } else {
        let volume = calculate_cone_volume(&r, &h);
        println!("Volume: {:.3}", volume);
    }
}
fn calculate_cone_volume(radius: &f64, height: &f64) -> f64 {
    (consts::PI * (radius * radius) * height) / 3.0
}
fn get_input(str: &str) -> f64 {
    println!("{}", str);
   let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: f64 = input.trim().parse().unwrap();
    num
}
