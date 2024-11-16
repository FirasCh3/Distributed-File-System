use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use lib::Block;
use std::fs::File;

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
            let mut buffer = String::new();
            match streams {
                Ok(mut stream) => {
                    stream.read_to_string(&mut buffer)?;
                    let mut deserialized: Block =  serde_json::from_str(buffer.as_str())?;
                    let content = deserialized.content();
                    if (content[0] == 0){
                        content.remove(0);
                        let filename = String::new() + "../resources/" + deserialized.filename() + deserialized.block_number().to_string().as_str() + ".txt";
                        let mut file = File::create(filename)?;
                        println!("{:?}", String::from_utf8(deserialized.content().clone()));
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
