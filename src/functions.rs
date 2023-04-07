// Functions -> Used to store blocks of code for re-use

pub fn run() {
    greetin("hello", "James");

    // Bind fumctions value to variables
    let get_sum = add(10, 5);
    println!("sum: {}", get_sum);

    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 2));
}

fn greetin(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    // return the number
    return num1 + num2;
}
