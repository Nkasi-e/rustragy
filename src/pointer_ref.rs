// Reference pointers - Basically points to a resource in memory
// Basically if we have a primitive array, we can create a variable and point to another variable, not just an array but any primitive value.

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Array values: {:?}", (arr1, arr2));

    //  Vector is non primitive
    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Vector values: {:?}", (&vec1, vec2));
}
