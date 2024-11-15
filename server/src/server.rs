use std::fs::File;
use std::io::Write;
use std::os::windows::fs::FileExt;
use std::path::Path;
use lib::Block;
use std::net::TcpStream;

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
        let mut  content1:Vec<u8> = vec![0; 1024];
        let mut  content2:Vec<u8> = vec![0; 1024];
        let mut  content3:Vec<u8> = vec![0; 1024];
        let file_path = Path::new(path);
        let mut file = File::open(file_path).unwrap();
        file.seek_read(&mut content1, offset).unwrap();
        offset = offset + (content1.len() as u64);
        file.seek_read(&mut content2, offset).unwrap();
        offset = offset + (content2.len() as u64);
        file.seek_read(&mut content3, offset).unwrap();
        offset = offset + (content3.len() as u64);
        let block1  = serde_json::to_string(&Block::new(content1, 1, String::from(file_path.file_name().unwrap().to_str().unwrap()))).unwrap();
        let block2  = serde_json::to_string(&Block::new(content2, 2, String::from(file_path.file_name().unwrap().to_str().unwrap()))).unwrap();
        let block3  = serde_json::to_string(&Block::new(content3, 3, String::from(file_path.file_name().unwrap().to_str().unwrap()))).unwrap();
        for slave_address in &self.slaves{
                let mut tcp_stream = TcpStream::connect(slave_address).unwrap();
                tcp_stream.write_all(&block1.as_bytes()).unwrap()
        }




    }

}
