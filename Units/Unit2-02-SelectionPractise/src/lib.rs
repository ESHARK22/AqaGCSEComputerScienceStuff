use InputHandlers::yes_no_input;

pub fn main() {
    println!("Task:
    Write a program to measure a user's resting heart rate.

    Ask the user: “Have you been at rest for at least 20 mins?”

    If the user inputs “Y”, ask the user “Is your pulse rate between 60 and 100 bpm?”

    If their pulse is between 60 and 100 bpm, print “Your heart rate is within the expected range.” Otherwise print “Your resting heart rate is outside of the expected range. You may wish to seek further advice from a professional.”
    If the user inputs “N”, print “Rest for 20 mins and try again!”

    Your program must include comments and you may need to use nested selection statements (ifs, elifs and else within ifs, elifs and else statements)

    Attach your Python Code to this Assignment.
    ");

    let has_rested = yes_no_input("Have you been at rest for at least 20 mins? (y/n)");

    if !has_rested {
        // They have not rested, so tell the user to come back later
        println!("Rest for 20 mins and try again!");
        return;
    }

    // They have rested, so ask them about their heart rate
    let has_normal_pulse = yes_no_input("Is your pulse rate between 60 and 100 bpm?");
    if has_normal_pulse {
        println!("Your heart rate is within the expected range.")
    } else {
        println!("Your resting heart rate is outside of the expected range. You may wish to seek further advice from a professional.")
    }
}
