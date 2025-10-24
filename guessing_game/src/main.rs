use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Uncomment the line below for easy debugging during development
    // println!("(Psst... the secret number is: {})", secret_number);

    loop {
        println!("Please input your guess.");

        // Create a mutable string to store the user's input
        let mut guess = String::new();

        // Read a line from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string to a number, handling invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop on a correct guess
            }
        }
    }
}
