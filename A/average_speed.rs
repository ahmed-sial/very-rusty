use std::io::{ self };

// Question: A car traveled for some hours. Take the number of Hours travelled as input on runtime,
// and then take input for the Distance Travelled during this time, Calculate Average Speed
// and display it on the screen.

fn main() {
    let distance = take_input("Enter the distance travelled: ");
    let duration = take_input("Enter the driving duration: ");
    if distance < 0.0 {
        println!("error: distance can't be negative.");
    } else if duration <= 0.0 {
        println!("error: duration can't be negative or zero.");
    } else {
        println!("Average speed: {:.2}", distance / duration);
    }
}

fn take_input(str: &str) -> f64 {
    println!("{}", str);
    let x: f64 = loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                println!("error: {}", err);
                input.clear();
                continue;
            }
        }
        match input.trim().parse() {
            Ok(n) => {
                break n;
            }
            Err(err) => {
                println!("error: {}", err);
                input.clear();
                continue;
            }
        }
    };
    x
}
