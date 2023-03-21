// Variables hold primitive data or preferences to data
// Variables are immutable by default -> means it can't be reassign
// Rust is a block-scoped language

pub fn var() {
    let name = "Nkasi";

    // Add the "mut" keyword to be able to mutate the variable, without the mut keyword the variable is immutable by default
    let mut age = 10;
    println!("My name is {} and I am {} years old", name, age);

    age = 40;

    println!("My name is {} and I am {} years old", name, age);

    // Define constant -> when using const keyword you need to explicitly define a type and const doesn't change
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // Assigning multiple variables at once
    let (my_name, my_age) = ("Nkasi", 30);
    println!("My name is {1} and my age is {0}", my_age, my_name);
}
