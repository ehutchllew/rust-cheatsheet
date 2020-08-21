/*
 * Vectors -- resizeable arrays.
*/

use std::mem;

pub fn run() {
    let mut numbers: Vec<u8> = vec![0, 123, 55, 27, 255];

    // Print All
    println!("OG Vec: {:?}", numbers);

    // Get Single Val
    println!("Last Value: {}", numbers[numbers.len() - 1]);

    // Re-Assign Val
    numbers[2] = 69;
    println!("New Vec: {:?}", numbers);

    // Add on to Vector
    numbers.push(6);
    numbers.push(231);
    println!("Pushed Vector: {:?}", numbers);

    // Pop Value
    numbers.pop();
    println!("Popped Vector: {:?}", numbers);

    // Vec are Stack Allocated
    println!("Vec Occupies: {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[u8] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through Vec Values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & Mutate
    for x in numbers.iter_mut() {
        if *x < 255 - 2 {
            *x += 2;
        }
    }
    println!("Mutated Vec Loop: {:?}", numbers);
}
