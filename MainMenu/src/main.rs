use dialoguer::{theme::ColorfulTheme, Select};
use InputHandlers::input;

// Projects
use Connect4;
use EasterChallenges;

// Units
use Unit2_02_SelectionPractise;
use Unit2_03_DefensiveDesignA;
use Unit2_03_DefensiveDesignB;

fn main() {
    let available_options = ["Units", "Projects", "Exit"];

    loop {
        let selection_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What section would you like to explore?")
            .default(0)
            .items(&available_options[..])
            .interact()
            .unwrap();

        match selection_index + 1 {
            1 => unit_handler(),
            2 => project_handler(),
            3 => {
                println!("Goodbye!");
                return;
            }
            _ => panic!("You're not meant to reach here!"),
        }
    }
}

fn project_handler() {
    loop {
        let projects = ["Easter Challenges", "Connect 4 Game", "Go back"];

        let project_selection_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the project you would like to run")
            .default(0)
            .items(&projects[..])
            .interact()
            .unwrap();

        match project_selection_index + 1 {
            1 => EasterChallenges::main(),
            2 => Connect4::main(),
            3 => return,
            _ => panic!("You're not meant to reach here!"),
        }

        enter_to_continue();
    }
}

fn unit_handler() {
    loop {
        let units = [
            "Unit 2.2 - Selection Practise",
            "Unit 2.3 - Defensive Design (A)",
            "Unit 2.3 - Defensive Design (B)",
            "Go back",
        ];

        let unit_selection_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the unit you would like to run")
            .default(0)
            .items(&units[..])
            .interact()
            .unwrap();

        match unit_selection_index + 1 {
            1 => Unit2_02_SelectionPractise::main(),
            2 => Unit2_03_DefensiveDesignA::main(),
            3 => Unit2_03_DefensiveDesignB::main(),
            4 => return,
            _ => panic!("You're not meant to reach here!"),
        }

        enter_to_continue();
    }
}

fn enter_to_continue() {
    input("Press enter to continue");
}
