use std::io; // Shortens "std::io::stdin()" to "io::stdin()".
use std::cmp::Ordering;
use rand::Rng;

/* Must be public to be called in main.rs. */
pub fn guess() {
    println!("Guess the number!");
    println!();

    let secret_num = rand::thread_rng().gen_range(1..=100); // Generate a random integer from 1-100.

    loop {
        println!("Please input your guess.");

        /* Variables are immutable by default in rust, and require the mut tag to become mutable. */
        let mut guess = String::new(); // Since String isn't a primitive type, it must be initialized via "String::new();".

        io::stdin() // Wait for user input
            .read_line(&mut guess) // Store the line in the mutable memory reference to guess (&guess is immutable by default).
            .expect("Failed to read line"); // In case of .read_line(&mut guess) returning an Error result, print "Failed to read line" and crash.

        if guess.contains("quit") { // Add way to exit the gameplay loop by typing "quit".
            break;
        }

        let guess: u32 = match guess.trim().parse() { // Parse the guess String into an unsigned 32-bit number.
            Ok(num) => num, // If completed without error, returning an unsigned 32-bit number with it, forward the number to the guess u32 variable.
            Err(_) => continue, // If an error is encountered, start a new loop iteration, ignoring the rogue input.
        };

        println!("You guessed: {guess}"); // Curly braces in a string ("") are placeholders that represent a variable.

        match guess.cmp(&secret_num) { // Matches are the same as switches in C-based languages.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { // If the guess is the same as the secret number, print "You win!", then break the loop (ending the game).
                println!("YOU WIN!");
                break;
            }
        }

        println!();
    }
    
}
