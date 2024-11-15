use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct Block{
    content: Vec<u8>,
    block_number: i32,
    filename: String,
}
impl Block{
    pub fn new(content: Vec<u8>, block_number: i32, filename: String) -> Self{
        Block{
            content,
            block_number,
            filename
        }
    }

    pub fn content(&self) -> &Vec<u8> {
        &self.content
    }

    pub fn block_number(&self) -> i32 {
        self.block_number
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}