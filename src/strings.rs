// Primitive str = Immutable fixed-length string somewhere in the memory
// string = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("hello ");

    // Get length -> can work for both string type
    println!("Length: {}", hello.len());

    // Push string
    hello.push('e'); // push for char type

    hello.push_str("choic"); //push_str for string type

    //  Capacity in bytes -> check the capacity using capacity method
    println!("Capacity: {}", hello.capacity());

    // checking to see if the string is empty using is_empty method
    println!("is empty: {}", hello.is_empty());

    // Checking if it cotains some substrings using contain method
    println!("Contains 'world' {}", hello.contains("world"));

    // Replace -> using the replace method
    println!("Replcace {}", hello.replace("hello", "hey"));

    // Loop through string by whitespace -> usign for loop
    for word in hello.split_whitespace() {
        println!("word {}", word);
    }

    // create string with capacity
    let mut name = String::with_capacity(10);
    name.push('N');
    name.push('K');

    println!("name {}", name);

    // How to write assertions to test basically -> it basically compares the first value to the second value and throws an error if it fails
    assert_eq!(2, name.len());
    assert_eq!(10, name.capacity());

    println!("{} rusticacians", hello);
}
