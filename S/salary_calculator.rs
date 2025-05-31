use std::io::{ self };
// Question: A customer asks the IT firm to develop a program, which can take tax rate
// and salary from the user on runtime and then calculate the tax, the user must pay and the
// salary he/she will have after paying the tax. Display the net income after tax deduction.

fn main() {
    let salary = take_input("Enter the salary: ");
    let tax = take_input("Enter the tax rate (in percentage): ");

    if salary <= 0.0 {
        panic!("error: negative or zero salary.")
    } else if tax < 0.0 {
        panic!("error: negative tax percentage.")
    } else {
        let net_salary = calculate_salary(&salary, &tax);
        println!("Net salary: {}", net_salary)
    }
}
fn calculate_salary(salary: &f64, tax_percent: &f64) -> f64 {
    let tax_salary = (tax_percent / 100.0) * salary;
    salary - tax_salary
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

