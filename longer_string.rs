fn main() {
    let str1 = "Hello";
    let str2 = "Hi";
    let result = longer_str(&str1.to_string(), &str2.to_string());
    println!("Longer string is \"{}\" having length {}.", result, result.len());
}
fn longer_str(str1: &String, str2: &String) -> String {
    if str1.len() > str2.len() {
        return str1.to_string();
    }
    str2.to_string()
}
