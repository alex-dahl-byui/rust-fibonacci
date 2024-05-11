use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        // set up a mutable variable to hold the user's input
        let mut input = String::new();
        let mut output = String::new();

        // print line asking the user for the fibonacci position
        print!("Show the Fibonacci sequence to what position? ");
        stdout().flush().unwrap();

        // read the user's input
        stdin().read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace and convert input to unsigned integer
        let position: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Ask for input again
            }
        };

        // set the loop's current position to 1
        let mut current_position = 1;
        // start a loop to find all the numbers to the specified position
        loop {
            if current_position == position {
                // if the current position is the position desired, insert the current position's fibonacci number to the output, with out a comma, and stop the loop.
                output += &format!("{}", find_fibonacci_placement(current_position));
                break;
            }
            else {
                // other wise insert the current position's fibonacci number to the output with a comma
                output += &format!("{}, ", find_fibonacci_placement(current_position));
            }

            // increment the current position by 1
            current_position += 1;
        }   
   
        // print the fibonacci sequence
        println!("Fibonacci sequence up to {}: {}", position, output);
        break; // Exit the loop
    }
}

fn find_fibonacci_placement(position: u64) -> u64 {
    // the nth fibonacci number can be found using the following formula: F(n)= (ϕ^n − ψ^n) / sqrt(5)
    let sqrt_5 = 5f64.sqrt();
    let phi = (1f64 + sqrt_5) / 2f64; // deriving the value of phi(ϕ)
    let psi = (1f64 - sqrt_5) / 2f64; // deriving the value of psi(ψ)

    ((phi.powi(position as i32) - psi.powi(position as i32)) / sqrt_5).round() as u64
}