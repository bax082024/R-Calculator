use std::io;

fn main() {
    println!("Welcome to my first Rust Calculator !");

    // Infinity loop
    loop {
        // Menu
        println!("\nChoose an operation");
        println!("1. Add (+)");
        println!("2. Subtract (-)");
        println!("3. Multiply (*)");
        println!("4. Divide (/)");

        // Read user choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        // convert input to number
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 5.");
                continue;
            }

        };


    }



}
