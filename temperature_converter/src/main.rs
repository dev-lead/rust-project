use std::io;

/**
 * Convert temperatures between Fahrenheit and Celsius.
 */
fn main() {
    //Start the program
    println!("Convert temperatures between Fahrenheit and Celsius.");
    println!("Select a function: (Enter the number)\n1. Fahrenheit to Celsius\n2. Celsius to Fahrenheit");

    // Get the function selection from the user.
    let mut function_selection = get_user_input();

    // Validate the function selection.
    if function_selection != 1 && function_selection != 2 {
        println!("\nInvalid selection! Please select a valid function.");
        function_selection = get_user_input();
    }

    // Get the temperature input from the user.
    if function_selection == 1 {
        println!("\nEnter the temperature in Fahrenheit:");
        let fahrenheit = get_user_input();

        let celsius = fahrenheit_to_celsius(fahrenheit as f32);
        println!("\n{} Fahrenheit is {:.2} Celsius.", fahrenheit, celsius);
    } else if function_selection == 2 {
        println!("\nEnter the temperature in Celsius:");
        let celsius = get_user_input();

        let fahrenheit = celsius_to_fahrenheit(celsius as f32);
        println!("\n{} Celsius is {:.2} Fahrenheit.", celsius, fahrenheit);
    }
}

/**
 * Get the user's input
 */
fn get_user_input() -> u32 {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => (),
        Err(_) => {
            println!("\nCan't understand your input. Try again!");
            return get_user_input(); //Restart
        }
    };

    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("\nPlease enter a valid number! {}", error);
            get_user_input() //Restart
        }
    };

    user_input
}

// Calculate the temperature in Celsius.
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Calculate the temperature in Fahrenheit.
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}
