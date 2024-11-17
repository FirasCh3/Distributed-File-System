use std::fmt::Binary;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct Message {
    content: String,
    block_number: i32,
    filename: String,
    operation_type: u8,
}
impl Message {
    pub fn new(content: String, block_number: i32, filename: String, operation_type: u8) -> Self{
        Message {
            content,
            block_number,
            filename,
            operation_type
        }
    }



    pub fn block_number(&self) -> i32 {
        self.block_number
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn operation_type(&self) -> u8 {
        self.operation_type
    }
}