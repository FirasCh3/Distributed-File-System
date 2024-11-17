use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use lib::Message;
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
        let mut offset: usize = 0;
        let mut  content:String = String::new();
        let file_path = Path::new(path);
        let mut file = File::open(&file_path).unwrap();

        file.read_to_string(&mut content).unwrap();
        let content1 = &content[0..content.len()/3];
        offset = offset + (content.len()/3);
        let content2 = &content[offset..(content.len()/3)*2];
        offset = offset + (content.len()/3);
        let content3 = &content[offset..];


        let message1  = serde_json::to_string(&Message::new(content1.to_string(), 1, String::from(file_path.with_extension("").file_name().unwrap().to_str().unwrap()), 0)).unwrap();
        let message2  = serde_json::to_string(&Message::new(content2.to_string(), 2, String::from(file_path.with_extension("").file_name().unwrap().to_str().unwrap()), 0)).unwrap();
        let message3  = serde_json::to_string(&Message::new(content3.to_string(), 3, String::from(file_path.with_extension("").file_name().unwrap().to_str().unwrap()), 0)).unwrap();
        for slave_address in &self.slaves{
                let mut tcp_stream = TcpStream::connect(slave_address).unwrap();
                tcp_stream.write_all(message1.as_bytes()).unwrap()
        }




    }

}
