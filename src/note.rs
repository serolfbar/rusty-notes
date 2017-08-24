#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: u32,
    content: String,
}

impl Note {
    pub fn new(last_id: u32, content: String) -> Note {
        Note {
            id: last_id + 1,
            content: content,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn print_note(&self) {
        println!("Note : {}", self.id);
        println!("{}", self.content);
        println!("-------------------------------------------");
    }
}
