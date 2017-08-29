use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;

const FILE_NAME: &str = "rustynotes.txt";

pub fn get_note_file() -> Result<File, Error> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(FILE_NAME)
}
pub fn erase_file_content() {
    let mut result = OpenOptions::new().truncate(true).write(true).open(
        FILE_NAME,
    );
    match result {
        Ok(mut file) => file.write_all(b""), 
        Err(err) => panic!("{:?}", err),
    };

}
