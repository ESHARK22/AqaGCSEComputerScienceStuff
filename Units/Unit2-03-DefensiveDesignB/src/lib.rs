use rand::{thread_rng, Rng};
use InputHandlers::int_input;

pub fn main() {
    // Charlie is developing an adding game. The rules of the game are:
    //  • the player is asked 3 addition questions
    //  • each question asks the player to add together two random whole numbers between 1 and 10 inclusive
    //  • if the player gets the correct answer, 1 is added to their score
    //  • at the end of the game their score is displayed.

    let mut score: i8 = 0;
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
