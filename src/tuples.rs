/*
 * Group of values -- can be diff types: (12, true, 'someStr')
 * Max 12 elements
*/

pub fn run() {
    let person: (&str, &str, i8) = ("Evan", "La Paz", 30);

    println!(
        "{} is from {} and is {} years old",
        person.0, person.1, person.2
    );
}
