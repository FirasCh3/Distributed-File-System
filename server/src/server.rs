use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use lib::Message;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

pub struct Server {
    slaves: [SocketAddrV4; 3],
}

impl Server {
    pub fn new()-> Server{
        Server{
           slaves: [
               SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4200),
               SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4201),
               SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4202)
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


        let message1:String  = serde_json::to_string(&Message::new(content1.to_string(), 1, String::from(file_path.with_extension("").file_name().unwrap().to_str().unwrap()), 0)).unwrap();
        let message2:String  = serde_json::to_string(&Message::new(content2.to_string(), 2, String::from(file_path.with_extension("").file_name().unwrap().to_str().unwrap()), 0)).unwrap();
        let message3:String  = serde_json::to_string(&Message::new(content3.to_string(), 3, String::from(file_path.with_extension("").file_name().unwrap().to_str().unwrap()), 0)).unwrap();
        let messages:Vec<String> = vec![message1, message2, message3];
        for (index, address)  in self.slaves.iter().enumerate(){
            let mut tcp_stream = TcpStream::connect(address).unwrap();
            tcp_stream.write_all(messages[index].as_bytes()).unwrap()
        }




    }

}
