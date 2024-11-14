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
}