use std::fs::File;
use std::os::windows::fs::FileExt;
use std::path::Path;
use lib::Block;
pub struct Server {
    slaves: [String; 3],
}

impl Server {
    pub fn new()-> Server{
        Server{
           slaves: [
               String::from("127.0.0.1:4200"),
               String::from("127.0.0.1:4201"),
               String::from("127.0.0.1:4202")
           ]
        }
    }
    pub fn store_file(&self, path: &str) {
        let mut offset: u64 = 0;
        let mut  block1:Vec<u8> = vec![0; 1024];
        let mut  block2:Vec<u8> = vec![0; 1024];
        let mut  block3:Vec<u8> = vec![0; 1024];
        let mut file = File::open(Path::new(path)).unwrap();
        file.seek_read(&mut block1, offset).unwrap();
        offset = offset + (block1.len() as u64);
        file.seek_read(&mut block2, offset).unwrap();
        offset = offset + (block2.len() as u64);
        file.seek_read(&mut block3, offset).unwrap();
        offset = offset + (block3.len() as u64);
        println!("{:?}", block1);
        let block : Block = Block::new();

    }

}
