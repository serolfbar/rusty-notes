use std::fs::OpenOptions;
use std::io::Write;

pub fn add_note(note: String) {
    let file_result = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("rustynotes.txt");

    match file_result {
        Ok(mut f) => f.write_all(note.as_bytes()),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

pub fn remove_note(note: Option<String>, id: Option<String>) {}

pub fn list_notes() {}

pub fn help() {}
