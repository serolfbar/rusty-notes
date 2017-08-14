extern crate serde;
extern crate serde_json;

use std::fs::OpenOptions;
use std::io::Write;
use note::Note;

pub fn add_note(note: Note) {
    let file_result = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("rustynotes.txt");

    match file_result {
        Ok(mut f) => {
            let json_note = serde_json::to_string(&note).unwrap();
            f.write_all(json_note.as_bytes());
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

/*
 * If the two parameters are available, the id and and note
 * must match for it to work.
 */
pub fn remove_note(note: Option<String>, id: Option<String>) {

    if note.is_some() && id.is_some() {
        //MUST POINT TO THE SAME THING
    }
    //    else {
    //        match note {
    //            Some(n) => ,
    //            None => ,
    //        };
    //        match id {
    //            Some(i) => ,
    //            None => ,
    //        }
    //    }
}

pub fn list_notes() {}

pub fn help() {}
