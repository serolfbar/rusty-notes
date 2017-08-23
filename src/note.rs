#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: u32,
    content: String,
}

impl Note {
    pub fn new(id: u32, content: String) -> Note {
        Note {
            id: id,
            content: content,
        }
    }
    pub fn print_note(&self) {
        println!("Note : {}", self.id);
        println!("{}", self.content);
        println!("-------------------------------------------");
    }
}
