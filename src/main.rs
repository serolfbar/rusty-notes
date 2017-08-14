#[macro_use]
extern crate serde_derive;

pub mod commands;
pub mod note;

use note::Note;

const ADD_COMMAND: &str = "add";
const REMOVE_COMMAND: &str = "remove";
const LIST_COMMAND: &str = "list";
const HELP_COMMAND: &str = "help";


/*
 * Arguments should be like following: <command> <param>
 * Exemple : rusty-notes add "Hello World"
 */
fn parse_arguments(mut args: Vec<String>) {
    let args_clone = args.clone();
    let command = args.remove(0);

    if first_command_is_valid(command.trim()) {
        //Maybe better way to fetch command and parameter of command
        let param = args.remove(0);

        match command.trim() {
            ADD_COMMAND => {
                let note = Note::new(0, param);
                commands::add_note(note);
            }
            REMOVE_COMMAND => commands::remove_note(None, None),
            LIST_COMMAND => commands::list_notes(),
            HELP_COMMAND => commands::help(),
            _ => {
                println!("Pleaser enter a valid command");
                println!("Refer to this to help you.");
                commands::help();
            }
        };
    }
}

fn first_command_is_valid(command: &str) -> bool {
    let is_valid = match command {
        ADD_COMMAND => true,
        REMOVE_COMMAND => true,
        LIST_COMMAND => true,
        HELP_COMMAND => true,
        _ => false,
    };
    is_valid
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    parse_arguments(args[1..].to_vec());
}
