use std::fs;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use lib::Message;
use std::fs::{File, OpenOptions};

#[derive(Debug)]
pub struct Slave {
    address: SocketAddrV4,
    port: u16,
}
impl Slave {
    pub fn new(port: u16) -> Slave {
        Slave {
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port),
            port,
        }
    }
    pub fn listen_and_serve(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address)?;
        for streams in listener.incoming() {
            let mut buffer:Vec<u8> = Vec::new();
            match streams {
                Ok(mut stream) => {
                    stream.read_to_end(&mut buffer)?;
                    let mut deserialized: Message =  bincode::deserialize(buffer.as_slice()).unwrap();
                    if (deserialized.operation_type() == 0){
                        let filename = "../resources/data.bin";
                        let mut file = OpenOptions::new().append(true).open(filename)?;
                        bincode::serialize_into(file, &deserialized).unwrap()
                    }else {
                        let block_number = deserialized.block_number();
                        //TODO: read block from data.bin file and send it back to server
                    }

                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }
}
