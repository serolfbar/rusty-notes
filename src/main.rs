#[macro_use]
extern crate serde_derive;

pub mod commands;
pub mod note;

use note::Note;

const ADD_COMMAND: &str = "add";
const REMOVE_COMMAND: &str = "remove";
const LIST_COMMAND: &str = "list";
const HELP_COMMAND: &str = "help";

fn parse_arguments(mut args: Vec<String>) {
    let args_clone = args.clone();
    let command = &args.remove(0)[..];

    let argument = if args.len() >= 1 {
        Some(args.remove(0))
    } else {
        None
    };

    if first_command_is_valid(command) {

        match command {
            ADD_COMMAND => {
                match argument {
                    Some(note_content) => {
                        let note = Note::new(0, note_content);
                        commands::add_note(note);
                        println!("You have added a new note.");
                    }
                    None => println!("Please enter a note."),
                };
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

    if args.len() > 1 {
        args.remove(0);
    }
    parse_arguments(args.to_vec());
}
