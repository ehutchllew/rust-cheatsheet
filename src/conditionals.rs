/*
 * Like any other language
 * No ternary
*/
pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    // If/Else
    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry kid, too young.");
    } else {
        println!("Bartender: I need to see your ID.");
    }

    // Shorthand If (sort of ternary)
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age)
}
