//  Vector is a continous growable array type

use std::mem;

pub fn vector() {
    // declaring a vector, that takes in the datatype and the length of the array
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Re-assigning values
    numbers[2] = 20;

    // Add on to vector
    numbers.push(9);
    numbers.push(10);

    // Remove value with the pop method
    numbers.pop();

    // Print all the values in vector using the debug trait
    println!("Vector: {:?}", numbers);

    // Get/Print a single value in a vector
    println!("single value: {}", numbers[2]);

    // Getting the length Vector
    println!("Vector length: {}", numbers.len());

    // Getting the amount of memory that it takes up -> Vector are stack allocated
    println!("This Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice in an array -> using the slice method
    let slice: &[i32] = &numbers[0..6]; // reference to numbers array
                                        // Note: the number[0..6] means we want to get from 0 to 6 in thvector of numbers.
    println!("slice: {:?}", slice);

    // Loop through a vector value
    for x in numbers.iter() {
        println!("Number: {:?}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2; // loop and multiply each value by 2
    }

    println!("Numbers Vec: {:?}", numbers);
}
