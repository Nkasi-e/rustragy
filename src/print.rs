pub fn run() {
    // Print to console
    println!("Print from the print.rs file");

    // Basic Formmating

    // how to print an integer or number
    println!("Number {}", 1);

    // holding or printing with placeholders {}
    println!("{} is a rust {}", "Nkasi", "programmer");

    // Positional Arguments
    println!(
        "{0} is from {1}, and {0} likes to {2}",
        "Nkasi", "Nigeria", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {sports}",
        name = "Nkasi",
        sports = "football"
    );

    // Placeholder traits -> prints out the binary, hex and octal value of the numbers
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits -> takes in various datatypes
    println!("{:?}", (12, true, "rust-lang"));

    // Basic Maths
    println!("10 + 10 = {}", 10 + 10);
}
