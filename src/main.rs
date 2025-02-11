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

        


    }



}
