use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    // Create an infinite loop so we can keep calculating until the user quits
    loop {
        // Show menu
        println!("\nChoose an operation:");
        println!("1. Add (+)");
        println!("2. Subtract (-)");
        println!("3. Multiply (*)");
        println!("4. Divide (/)");
        println!("5. Exit");

        // Read user choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        // Convert input to a number
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 5.");
                continue;
            }
        };

        // Handle user choice
        match choice {
            1 => perform_operation("+"),
            2 => perform_operation("-"),
            3 => perform_operation("*"),
            4 => perform_operation("/"),
            5 => {
                println!("Goodbye! 👋");
                break;
            }
            _ => println!("Invalid choice! Please select a number between 1 and 5."),
        }
    }
}

fn perform_operation(operator: &str) {
    let num1 = get_number("Enter the first number: ");
    let num2 = get_number("Enter the second number: ");

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}

