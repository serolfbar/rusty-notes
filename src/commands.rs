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

    if id <= 0 {
        panic!("The id for a note should be higher than zero.");
    }
    if note_content.is_empty() {
        panic!("The note should be provided a valid note content.");
    }

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

pub fn number_of_notes(file: &mut File) -> usize {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let lines = buffer.lines();
    lines.count()
}



#[cfg(test)]
mod tests {
    extern crate tempdir;
    use super::*;
    use self::tempdir::TempDir;

    fn create_temp_dir_and_file() -> (TempDir, File) {
        let dir = match TempDir::new("test_directory") {
            Ok(directory) => directory,
            Err(err) => panic!("Temporary directory could not be created"),
        };
        let file_path = dir.path().join("test_notes.txt");

        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(err) => panic!("Could not create a testing file"),
        };
        (dir, file)
    }

    #[test]
    #[should_panic(expected = "The id for a note should be higher than zero.")]
    fn adding_a_note_without_id_should_panic() {
        let id = 0;
        let (dir, mut file) = create_temp_dir_and_file();

        add_note(&mut file, String::from("This is the content"), id);
        dir.close();
    }

    #[test]
    #[should_panic(expected = "The note should be provided a valid note content.")]
    fn adding_a_note_without_content_should_panic() {
        let (dir, mut file) = create_temp_dir_and_file();
        add_note(&mut file, String::from(""), 1);
        dir.close();
    }

    #[test]
    fn added_note_should_be_in_file() {
        let (dir, mut file) = create_temp_dir_and_file();
        let number_notes = number_of_notes(&mut file);
        assert_eq!(1, number_notes);
        dir.close();
    }
}
