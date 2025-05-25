fn main() {
    let str1 = "Hello";
    let str2 = "Hi";
    let result = longer_str(&str1, &str2);
    println!("Longer string is \"{}\" having length {}.", result, result.len());
}
fn longer_str<'a>(str1: &'a str, str2: &'a str) -> &'a str{
    if str1.len() > str2.len() {
        return str1;
    }
    str2
}
