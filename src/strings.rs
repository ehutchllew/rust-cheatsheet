/*
 * Primitive str = Immutable fixed-length string somewhere in memory
 * String = Growable, heap-allocated data structure -- use to modify str data
*/
pub fn run() {
    // Primitive
    let hello = "Hello";

    // String
    let mut dynamic_string = String::from("Hello ");

    // Get Length
    println!("Length: {} and {}", hello.len(), dynamic_string.len());

    // Push Char
    dynamic_string.push('W');

    // Push String
    dynamic_string.push_str("orld!");
    println!("{}", dynamic_string);

    // Capacity in Bytes
    println!("Capacity: {}", dynamic_string.capacity());

    // Check if Empty
    println!("Is Empty: {}", dynamic_string.is_empty());

    // Contains Substr
    println!("Contains 'World': {}", dynamic_string.contains("World"));

    // Replace
    println!("Replace: {}", dynamic_string.replace("World", "There"));

    // Loop Through Str by Whitespace
    for word in dynamic_string.split_whitespace() {
        println!("word: {}", word);
    }

    // Create Str with Capacity
    let mut s = String::with_capacity(10);
    s.push_str("ab");
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
