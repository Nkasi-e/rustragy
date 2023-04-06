pub fn run() {
    let age: u8 = 20;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // IF/ELSE
    // if age >= 20 {
    //     println!("Well you just clocked {} and you're now an adult", age);
    // } else {
    //     println!("You need to grow up real quick");
    // }
    if age >= 20 && check_id || knows_person_of_age {
        println!("Well you just clocked {} and you're now an adult", age);
    } else if age < 21 && check_id {
        println!("You need to grow up real quick");
    } else {
        println!("failed access");
    }

    // shorthand if/else
    let is_of_age = if age >= 22 { true } else { false };

    println!("Is of age: {}", is_of_age);
}
