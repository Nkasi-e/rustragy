// Loops are used to iterate until a condition is met
// In rust we have what is called an infinite loop

pub fn run() {
    let mut count = 0;

    // infinite loop

    loop {
        count += 1;
        println!("Number: {}", count); // this will keep running if no condition is given

        if count == 20 {
            break;
        }
    }

    // while loop (FizzBuzz)

    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("Number: {}", count);
        }

        // increament
        count += 1;
    }

    // For Range Loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("Number: {}", x);
        }
    }
}
