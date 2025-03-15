
use std::io;

fn main() {
    println!("This is a Fibonacci number generator");
    println!("Enter the number of Fibonacci numbers you want to generate: ");

    let mut input: u32 = get_user_input();
    if input == 0 {
        println!("You can't generate {} Fibonacci numbers. Try again!", input);
        input = get_user_input();
    }

    let mut fibonacci_sequence: Vec<u32> = Vec::new();
    fibonacci_sequence.push(0);
    fibonacci_sequence.push(1);
    get_fibonacci_sequence(input - 1, &mut fibonacci_sequence);
    let index: usize = (input - 1) as usize;
    let nth_fibonacci: u32 = fibonacci_sequence[index];

    println!("\nRunning sequence...\n{:?}", fibonacci_sequence);
    println!("The ({})th Fibonacci number is: {}", input, nth_fibonacci);
}

/**
 * Get the Fibonacci sequence
 */
fn get_fibonacci_sequence(n: u32, sequence: &mut Vec<u32>) -> u32 {
    let fibonacci: u32;

    //Check if key exists in the sequence already
    if (n as usize) < sequence.len() {
        fibonacci = sequence[n as usize];
    } else {
        //Get the last two Fibonacci numbers
        let last = sequence.len() - 1;
        let second_last = sequence.len() - 2;
        let last_fibonacci = sequence[last];
        let second_last_fibonacci = sequence[second_last];

        //Calculate the Fibonacci number
        sequence.push(last_fibonacci + second_last_fibonacci);

        //Recursively call the function
        fibonacci = get_fibonacci_sequence(n, sequence);
    }

    fibonacci
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