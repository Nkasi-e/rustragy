// Arrays -> Fixed list where elements are the same data types, in rust the length of an array is fixed
use std::mem;

pub fn arrays() {
    // declaring an array, that takes in the datatype and the length of the array
    let mut numbers: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    // Re-assigning values
    numbers[2] = 20;

    // Print all the values in an array using the debug trait
    println!("Arrays: {:?}", numbers);

    // Get/Print a single value in an array
    println!("single value: {}", numbers[2]);

    // Getting the length of the array
    println!("Array length: {}", numbers.len());

    // Getting the amount of memory that it takes up -> Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));
    // println!(
    //     "This array occupies {} bytes",
    //     std::mem::size_of_val(&numbers)
    // );

    // Get slice in an array -> using the slice method
    let slice: &[i32] = &numbers[0..6]; // reference to numbers array
                                        // Note: the number[0..6] means we want to get from 0 to 6 in the array of numbers.
    println!("slice: {:?}", slice);
}
