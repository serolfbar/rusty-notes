extern crate serde;
extern crate serde_json;

use std::fs::OpenOptions;
use std::io::Write;
use note::Note;
use note;
use std::fs::File;
use std::io::Error;
use std::str;
use std::io::Read;

const FILE_NAME: &str = "rustynotes.txt";

fn get_note_file() -> Result<File, Error> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(FILE_NAME)
}


//TODO Find better way to find last_id (Database maybe?)
fn get_last_id() -> u32 {
    let last_id: u32;
    let file_result = get_note_file();

    match file_result {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer);
            let last_line = buffer.lines().last();
            match last_line {
                Some(line) => {
                    let note: Note = serde_json::from_str(line).unwrap();
                    last_id = note.get_id();
                }  
                None => last_id = 0,
            }

        } 
        Err(e) => panic!("{:?}", e), 
    }

    last_id
}

pub fn add_note(note_content: String) {

    let file_result = get_note_file();
    let last_id = get_last_id();
    let note = Note::new(last_id, note_content);

    match file_result {
        Ok(mut f) => {
            let mut json_note = serde_json::to_string(&note).unwrap();
            json_note.push('\n');
            f.write_all(json_note.as_bytes());
        }
        Err(e) => panic!("{:?}", e),
    };
}

pub fn remove_note(note: Option<String>, id: Option<String>) {

    if note.is_some() && id.is_some() {}
}

pub fn list_notes() {
    let file_result = get_note_file();

    let file_content = match file_result {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer);

            let lines = buffer.lines();

            for line in lines {
                let note: Note = serde_json::from_str(line).unwrap();
                note.print_note();
            }

        }
        Err(err) => panic!("{}", err), 
    };
}

pub fn help() {
    println!("");
    println!("Usage : rusty-notes <command> [<args>]");
    println!("");
    println!("  add \"note\"   : Adds a new note which is between quotes.");
    println!("  list         : Prints all saved notes.");
    println!("  remove <id>  : Removes the note assigned to id.");
    println!("  help         : Prints help.");
}
