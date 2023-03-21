// Primitive str = Immutable fixed-length string somewhere in the memory
// string = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("hello ");

    // Get length -> can work for both string type
    println!("Length: {}", hello.len());

    hello.push('e'); // for char type

    hello.push_str("choic");
    println!("{} rusticacians", hello);
}
