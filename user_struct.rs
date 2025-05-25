#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}
impl User {
    fn is_active<'a>(&'a self) -> &'a str{
        if self.active == true {
            return "Active";
        }
        "InActive"
    }
}
fn main() {
    let user = User {
        username: String::from("Ahmed Hassan"),
        email: String::from("ahmed@gmail.com"),
        active: true,
    };
    println!("{:#?}", user);
    println!("User State: {}", user.is_active());
}
