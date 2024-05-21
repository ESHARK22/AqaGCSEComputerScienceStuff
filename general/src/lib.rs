pub mod input_handlers {
    use std::io::{
        stdin,
        stdout,
        Write
    };

    pub fn input(prompt: impl AsRef<str> + std::fmt::Display) -> String {

        loop {
            print!("{prompt}");

            // Flush stdout, since its a new line buffer, but we are not printing a new line
            if let Err(error) = stdout().flush() {
                // Print the error message and go to the next iteration
                println!("error: {error}", );
                println!("Try again...\n");
                continue
            }

            // Create a buffer, or clear the old one
            let mut inp_buffer = String::new();

            // Read stdin into the "buffer"
            match stdin().read_line(&mut inp_buffer) {
                Ok(_) => {
                    return inp_buffer;
                }
                Err(error) => {
                    // Print the error message and go to the next iteration
                    println!("error: {error}", );
                    println!("Try again...\n");
                    continue
                }
            }
        }
    }

    pub fn int_input(prompt: impl AsRef<str> + std::fmt::Display) -> usize {
        loop {
            let str_input = input(&prompt);

            let int_input = str_input.trim().parse::<usize>();
            match int_input {
                Ok(int) => {
                    return int;
                }
                Err(error) => {
                    println!("error: {error}");
                    println!("Try again...");
                    continue    // Try again
                }
            }

        }
    }

    pub fn yes_no_input(prompt: impl AsRef<str> + std::fmt::Display) -> bool {
        // Yes -> True
        // No  -> False

        loop {
            let str_input = input(&prompt)
                .trim()
                .to_lowercase();

            if str_input.as_str() == "y" || str_input.as_str() == "yes" {
                // Yes
                return true

            } else if str_input.as_str() == "n" || str_input.as_str() == "no" {
                // No
                return false

            } else {
                // Not a valid input
                println!("Invalid input!");
                println!("Please only enter either yes or no... \n")
            };
        }
    }

}