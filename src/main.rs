#[macro_use]
extern crate serde_derive;

pub mod commands;
pub mod note;
pub mod file_operations;

use note::Note;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::io::Read;

const ADD_COMMAND: &str = "add";
const REMOVE_COMMAND: &str = "remove";
const LIST_COMMAND: &str = "list";
const HELP_COMMAND: &str = "help";

fn get_last_id(file: &mut File) -> usize {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    buffer.lines().count()
}

fn parse_arguments(mut args: Vec<String>) {
    let args_clone = args.clone();
    let command = &args.remove(0)[..];

    let mut file = match file_operations::get_note_file() {
        Ok(f) => f,
        Err(err) => panic!("{:?}", err),
    };

    let argument = if args.len() >= 1 {
        Some(args.remove(0))
    } else {
        None
    };

    if command_is_valid(command) {
        match command {
            ADD_COMMAND => {
                match argument {
                    Some(note_content) => {
                        let id = get_last_id(&mut file) + 1;
                        commands::add_note(&mut file, note_content, id);
                        println!("You have added a new note.");
                    }
                    None => println!("Please enter a note."),
                };
            }
            REMOVE_COMMAND => {
                match argument {
                    Some(id) => {

                        println!("You removed note {}", id);
                        commands::remove_note(&mut file, id);
                        //TODO Add confirmation.
                    }
                    None => println!("Please enter a valid id."),

                }
            }
            LIST_COMMAND => commands::list_notes(&mut file),

            HELP_COMMAND => commands::help(),
            _ => {
                println!("Pleaser enter a valid command");
                println!("Refer to this to help you.");
                commands::help();
            }
        };
    }
}

fn command_is_valid(command: &str) -> bool {
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

    //TODO better way of doing this?
    //This will remove the first argument which is the environment.
    if args.len() > 1 {
        args.remove(0);
    }
    parse_arguments(args.to_vec());
}
