/*
 * Arrays -- Fixed list length, elements are of same type.
*/

use std::mem;

pub fn run() {
    let mut numbers: [u8; 5] = [0, 123, 55, 27, 255];

    // Print All
    println!("OG Array: {:?}", numbers);

    // Get Single Val
    println!("Last Value: {}", numbers[numbers.len() - 1]);

    // Re-Assign Val
    numbers[2] = 69;
    println!("New Array: {:?}", numbers);

    // Arrays are Stack Allocated
    println!("Array Occupies: {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[u8] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
