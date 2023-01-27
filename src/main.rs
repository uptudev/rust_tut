mod basics;
mod guessing_game;
fn main() {
    //Basic OOP, using the basics module to import methods written in ./basics.rs
    basics::hello_world();
    println!();

    loop {
        /* Prompt for input */
        print!("Please enter the number to find the inverse square root of: ");
        println!();

        /* Get input and bind it to a string named in_string */
        let mut in_string: String = String::new();
        std::io::stdin()
            .read_line(&mut in_string)
            .expect("Failed to read line");
        println!();

        /* Parse input string into a 64-bit float */
        let input: f64 = match in_string.trim().parse() {
            Ok(num) => num, // If completed without error, returning an unsigned 64-bit number with it, forward the number to the input f64 variable.
            Err(_) => continue, // Else reprompt.
        };

        let fl32: f32 = basics::q_rsqrt32(input as f32);
        let fl64: f64 = basics::q_rsqrt(input);
        let fl_real: f64 = basics::rsqrt(input);
        println!("32-bit approximation: \t\t{fl32}");
        println!("64-bit approximation: \t\t{fl64}");
        println!("Slow calculation (most precise):{fl_real}");
        println!();
        print!("Do another? (y/n): ");
        println!();

        std::io::stdin()
            .read_line(&mut in_string)
            .expect("Failed to read line");

        if in_string.contains("y") || in_string.contains("Y") {
            continue;
        } else if in_string.contains("n") || in_string.contains("N") {
            break;
        } else {
            println!("That wasn't an option >:(");
            println!();
            break;
        }
    }
    guessing_game::guess();
}
