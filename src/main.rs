use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");


    loop {
        println!("\nChoose an operation:");
        println!("1. Add (+)");
        println!("2. Subtract (-)");
        println!("3. Multiply (*)");
        println!("4. Divide (/)");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 5.");
                continue;
            }
        };

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

fn get_number(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);

        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number! Please enter a valid number."),
        }
    }
}


