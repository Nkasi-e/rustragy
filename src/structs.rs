// struct is very important in rust, there are similar to classes in Javascript and python

// Structs -> Basically used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Connection(u8, u8, u8);

// struct that has a function
struct Person {
    first_name: String,
    last_name: String,
}
// creating some function that is associated with the Person struct
// impl -> implementation(implement)
impl Person {
    // Construct person -> create a new person.
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // creating a method that gets the full name
    fn full_name(&self) -> String {
        // format!() is a macro and it is similar to println! except it doesn't actually print
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    // to mutate the name we add the &mut as a parameter
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // traditional struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    // tuple struct
    let mut d = Connection(255, 0, 0);

    d.0 = 100;

    println!("Connection: {}, {}, {}", d.0, d.1, d.2);

    let mut p = Person::new("Rust", "Dev");

    p.set_last_name("Developer");

    println!(
        "Implementing: I code in {} as a {}",
        p.first_name, p.last_name
    );

    println!("Info Person: {}", p.full_name());

    println!("Info Person tuple: {:?}", p.to_tuple());
}
