// for creating CLI commands

use std::env;

pub fn cli() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Nkasi";
    let status = "active";
    // println!("Args: {:?}", args);
    // println!("Args: {}", command);

    if command == "hello" {
        println!("Welcome {}, How may I help you?", name);
    } else if command == "status" {
        println!("You have an {1} status! {0}", name, status);
    } else {
        println!("command not valid! Please enter a valid command");
    }
}
