use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;
use std::env;

//TODO refactor this thing.
fn construct_path() -> String {
    let mut path_buf = env::current_exe().unwrap(); //Should be on the same directory as the runnable.
    let file_name = "/rustynotes.txt";
    let path = path_buf.as_path();
    let parent = path.parent();

    match parent {
        Some(p) => {
            let parent_as_string = p.to_str().unwrap();

            let mut path = String::from(parent_as_string);
            path.push_str(file_name);
            path
        }
        None => panic!("Theres a problem"),
    }
}

pub fn get_note_file() -> Result<File, Error> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(construct_path())
}

pub fn erase_file_content() {
    let mut result = OpenOptions::new().truncate(true).write(true).open(
        construct_path(),
    );
    match result {
        Ok(mut file) => file.write_all(b""), 
        Err(err) => panic!("{:?}", err),
    };
}
