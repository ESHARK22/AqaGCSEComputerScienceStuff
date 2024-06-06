use general::input_handlers::{
    input,
    int_input
};

use dialoguer::{
    theme::ColorfulTheme, 
    Select
};

use chrono::{
    self, 
    Local, 
    NaiveDate
};

fn main() {
    // task_1()
    let selections = &[
        "Task 1 - Number between 0-100",
        "Task 2 - Date of birth",
        "Task 3 - Password",
        "Exit"
    ];
    loop {
        let selection_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the task you would like to run")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        let selected_index = selection_index + 1;

        match selected_index {
                1 => { task_1()  }  
                2 => { task_2()  }  
                3 => { task_3()  }  
            
                4 => {
                    // Quit :(
                    println!("Goodbye!");
                    return 
                }
                _ => {
                    // Its none of the above?
                    println!("Wait a minuite...\n You're not meant to be here!")
                }

        }

        print!("\n    Press enter to return back to the main menu...");
        io::stdout().flush().unwrap_or({});
        io::stdin().read_line(&mut String::new()).unwrap_or(0);
    }
    
}


fn task_1() {
    // Write a program that asks the user to enter a number between 0 and 100 only. 
    loop {
        let number = int_input("Enter a number between 1 and 100: "); 

        if number > 100 {
            println!("Thats bigger than 100!");
            println!("Try again...\n");
            continue
        } else {
            println!("> {number} is a number between 1 and 100!");
            break
        }
    }
}

fn task_2() {
    // Write a program that asks for my Date of Birth. 

    let now = Local::now().date_naive();

    loop {

        let year  = int_input("Enter the year you were born [YYYY]: "   ) as i32;
        let month = int_input("Enter the month you were born in [MM]: " ) as u32;
        let day   = int_input("Enter the day you were born in [DD]: "   ) as u32;
       
        let dob = match NaiveDate::from_ymd_opt(year, month, day) {
            Some(dob) => dob,
            None => {
                println!("Invalid date!");
                println!("Try again...\n");
                continue
            }
        };

        // Make sure they were born before today, and calc their age
        let age = match now.years_since(dob) {
            Some(age) => age,
            None => {
                println!("You must be atleast 1 year old!!!");
                println!("Try again...\n");
                continue
            }
        };

        // Make sure they are not older than the oldest person ever
        if age > 122 {
            println!("You cant be older than the oldest person!");
            println!("Try again!\n");
            continue
        }

        println!("> {dob} is a valid date of birth!");
        println!("> Your are {age} years old!")
    }

}

fn task_3() {
    // Write a program that asks for a password that contains at least 1 Uppercase, a symbol 
    // and is longer that 8 characters. 
    
    loop {
        let password = input("Enter a password: ");

        // Make sure the password contains at least 1 uppercase letter   
        match password.chars().find(|&c| c.is_uppercase()) {
            None =>{
                println!("Your password must include at least 1 uppercase letter!");
                println!("Try again...\n");
                continue;
            }
            Some(_) => {}
        }

        // Make sure the password contains more than 1 special character
        let special_chars = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        match password.chars().find(|&c| special_chars.contains(c)) {
            None =>{
                println!("Your password must include at least 1 special charcter [{special_chars}]");
                println!("Try again...\n");
                continue;
            }        
            Some(_) => {}
        }

        // Make sure the password is atleast 8 chars long
        if !(password.len() >= 8) {
            println!("Your password must be atleast 8 characters long!");
            println!("Try again...\n");
            continue;        
        }
        
        // All checks passed
        break
    }

    println!("That is a valid password!");

}
