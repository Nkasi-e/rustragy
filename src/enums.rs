// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on Info
    match m {
        Movement::Up => println!("Moving upward"),
        Movement::Down => println!("Moving Downward"),
        Movement::Left => println!("Moving Left"),
        Movement::Right => println!("Moving Right"),
    } // Note: Match is similar to switch condition in Js
}

pub fn run() {
    let avatar1 = Movement::Right;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
