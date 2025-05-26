enum Day {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}
fn main() {
    println!("{}", typeOfDay(Day::SATURDAY));
}
fn typeOfDay(day: Day) -> &'static str{
    match day {
        Day::SATURDAY | Day::SUNDAY => "Weekend",
        _ => "Weekday",
    }
}
