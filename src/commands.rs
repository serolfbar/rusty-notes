extern crate serde;
extern crate serde_json;

use std::io::Write;
use note::Note;
use note;
use std::fs::File;
use std::io::Error;
use std::str;
use std::string::String;
use std::io::Read;
use file_operations;


pub fn add_note(file: &mut File, note_content: String, id: usize) {
    let note = Note::new(id, note_content);
    let mut json_note = serde_json::to_string(&note).unwrap();
    json_note.push('\n');

    file.write_all(json_note.as_bytes());
}

pub fn remove_note(file: &mut File, id_input: String) {
    let mut contents: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let mut id = 1;
    file.read_to_string(&mut buffer);

    let lines = buffer.lines();

    for line in lines {
        let mut note: Note = serde_json::from_str(line).unwrap();
        let id_as_number: usize = id_input.parse().unwrap();

        if id_as_number != note.get_id() {
            contents.push(note.get_content());
        }
    }
    file_operations::erase_file_content();

    for content in contents {
        add_note(file, content, id);
        id += 1;
    }
}

pub fn list_notes(file: &mut File) {

    let mut buffer = String::new();
    file.read_to_string(&mut buffer);

    let lines = buffer.lines();

    for line in lines {
        let note: Note = serde_json::from_str(line).unwrap();
        note.print();
    }
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
