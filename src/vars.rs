/**
 * Variables hold primitive data or references to data
 * Variables are immutable by default
 * Rust is a block-scoped language
 */

pub fn run() {
    let name = "Evan";
    let mut age = 30;
    println!("My name is: {}, and I am {} years old.", name, age);
    age = 31;
    println!("Age is now: {}", age);

    // Define Constant
    const ID: i8 = 100;
    println!("ID: {}", ID);

    // Assign Multiple Vars
    let (my_name, my_age) = ("Evan", 30);
    // Can do:
    // let (my_name, mut my_age)
    println!("{} is {}", my_name, my_age)
}
