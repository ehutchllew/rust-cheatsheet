/*
* Primitive Types:
 * Integers: u8, i9, u16, i16, u32, i32, u64, i64, u128, i128
 ** u = positive(u8 is 0 thru 255)
 ** i = pos/neg(i8 is -128 thru 127)
 * Floats: f32, f64
 * Boolean: (bool)
 * Characters: (char)
 * Tuples
 * Arrays -- fixed length
 * Vectors -- Arrays but variable length
*/

// Rust is a statically typed language, it must know types of all vars at compile.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add Explicit Type
    let max_u16: u16 = 65535;
    let min_i16: i16 = -32768;

    // Find Max Size
    println!("Min u16: {}", std::u16::MIN);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active = true;
    // Get Boolean From Expression
    let is_greater = 10 < 5;

    // Infer Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!(
        "{:?}",
        (x, y, max_u16, min_i16, is_active, is_greater, face, a1)
    )
}
