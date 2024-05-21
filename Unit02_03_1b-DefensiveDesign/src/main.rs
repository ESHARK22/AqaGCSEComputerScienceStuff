use std::io;
use std::io::Write;
use rand::{thread_rng, Rng};

fn main() {
    // Charlie is developing an adding game. The rules of the game are: 
    //  • the player is asked 3 addition questions 
    //  • each question asks the player to add together two random whole numbers between 1 and 10 inclusive 
    //  • if the player gets the correct answer, 1 is added to their score 
    //  • at the end of the game their score is displayed. 

    let mut score:i8 = 0;
    let mut rng = thread_rng();

    for x in 1..4 {
        let num1 = rng.gen_range(1..=10) as usize;
        let num2 = rng.gen_range(1..=10) as usize;
        let correct_ans = num1 + num2;

        let user_ans = int_input(&format!("Q.{x}) {num1} + {num2} = "));
        
        if user_ans != correct_ans {
            println!("Incorrect! The correct answer was {correct_ans}")
        } else {
            score += 1;
            println!("Correct! Your scorce is now {score}!")
        }
    }

    println!("You got a total score of {score}!")

}

fn input(prompt: &String) -> String {

    loop {
        print!("{prompt}");

        // Flush stdout, since its a new line buffer, but we are not printing a new line
        if let Err(error) = io::stdout().flush() {
            // Print the error message and go to the next iteration
            println!("error: {error}", );
            println!("Try again...\n");
            continue
        }

        // Clear the buffer
        let mut inp_buffer = String::new();

        // Read stdin into the "buffer"
        match io::stdin().read_line(&mut inp_buffer) {
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

fn int_input(prompt: &String) -> usize {
    loop {
        let str_input = input(&prompt);

        let int_input = str_input.trim().parse::<usize>();
        match int_input {
            Ok(int) => {
                return int;
            }
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Try again
            }
        }

    }
}