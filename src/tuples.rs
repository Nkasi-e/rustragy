// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Rust", "Lang", 47);

    println!("{} is a good {} and it is {}", person.0, person.1, person.2);
}
