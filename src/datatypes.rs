/**
   Primitive types--
   Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits tey take in memory)
   Floats: f32, f64
   Boolean: (bool)
   Characters: (char)
   Tuples
   Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to  use based on the value and how we use it.

pub fn types() {
    println!("Hello datatypes!");

    // Default is "i32"
    let _x = 1;

    // Default is "f64"
    let _y = 3.5;

    // Add explicit type
    let _z: i64 = 5954309349;

    // finding the max size of i32 or i64 types
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max f64: {}", std::f64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 > 20;

    // char data type -> it has to be a single character with single quote
    // it can be a unicode, like an emoji
    let fleet = 'a';
    let face = '\u{1F600}'; // smiley face emoji

    println!("{:?}", (_x, _y, _z, is_active, is_greater, fleet, face));
}
