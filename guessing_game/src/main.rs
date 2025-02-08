use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //Clear screen
    print!("\x1B[2J\x1B[1;1H");
    //io::stdout().flush().unwrap();

    println!("Are you a psychic? Guess the number (from 1 to 100) to find out!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut psychic_rank = 1;

    loop {
        let mut guess = String::new();
        println!("Please input your guess:");
        match io::stdin().read_line(&mut guess) {
            Ok(num) => num,
            Err(_) => {
                println!("Can't understand your guess!");
                continue; //Restart
            }
        };

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Please enter a valid number! {}", error);
                continue; //Restart
            }
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");

                match psychic_rank {
                    1 => println!("You are a psychic! You guessed the number in {} tries!", psychic_rank),
                    2 => println!("You have cosmic abilities but they need work! You guessed the number in {} tries!", psychic_rank),
                    3 => println!("You can use some work but there's potential there! You guessed the number in {} tries!", psychic_rank),
                    _ => println!("UMm... You couldn't see that coming? You guessed the number in {} tries!", psychic_rank),
                };

                break;

            }
        }

        if psychic_rank > 4 {
            println!("You are not a psychic! It took you {} tries!", psychic_rank);
            println!("The secret number was: {}", secret_number);
            break;
        }

        psychic_rank += 1;
        
    }
    
    

    
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Please enter a valid number!");
    //         return;
    //     }
    // };

    // println!("You guessed: {}", guess);

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }

    // println!("The secret number was: {}", secret_number);

    // if guess == secret_number {
    //     println!("You win!");
    // } else {
    //     println!("You lose! The secret number was: {}", secret_number);
    // }
}