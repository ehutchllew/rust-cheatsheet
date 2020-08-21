pub fn run() {
    // Basic Print
    println!("Hello from print.rs file");

    // Basic Formatting
    println!("Number: {}{}", 1, 2);

    // Positional Formatting
    println!("{0} is from {1} and {0} likes to {2}", "Evan", "LP", "Code");

    // Named Formatting
    println!(
        "{name} likes to play {activity}.",
        name = "Lucas",
        activity = "everything"
    );

    // Placeholder Traits
    println!(
        "Binary: {num:b} \n Hex: {num:x} \n Octal: {num:o}",
        num = 10
    );

    // Debug Traits
    println!("{:?}", (12, true, "something"));

    // Basic Math
    println!("{} + {} = {}", 10, 11, 10 + 11);
}
