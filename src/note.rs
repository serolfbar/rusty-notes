#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: usize,
    content: String,
}

impl Note {
    pub fn new(id: usize, content: String) -> Note {
        Note {
            id: id,
            content: content,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn print(&self) {
        println!("Note : {}", self.id);
        println!("{}", self.content);
        println!("-------------------------------------------");
    }
}
