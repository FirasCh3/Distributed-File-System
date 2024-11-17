use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use lib::Message;
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
            let mut string = String::new();
            match streams {
                Ok(mut stream) => {
                    stream.read_to_string(&mut buffer)?;
                    let mut deserialized: Message =  serde_json::from_str(buffer.as_str())?;
                    let content = deserialized.content();
                    if (deserialized.operation_type() == 0){
                        let filename = String::new() + "../resources/" + deserialized.filename() + deserialized.block_number().to_string().as_str() + ".txt";
                        File::options().read(true).write(true).create(true);
                        let mut file = File::create(filename)?;
                        file.write_all(deserialized.content().as_bytes())?;


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
