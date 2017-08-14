#[derive(Serialize, Deserialize)]
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
}
