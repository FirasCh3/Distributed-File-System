use std::io::{Read, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
use lib::Block;

#[derive(Debug)]
pub struct Slave {
    address: SocketAddrV4,
    port: u16,
}
impl Slave {
    pub fn new(port: u16) -> Slave {
        return Slave {
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port),
            port,
        };
    }
    pub fn listen_and_serve(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address)?;
        for streams in listener.incoming() {
            let mut buffer = String::new();
            match streams {
                Ok(mut stream) => {
                    stream.read_to_string(&mut buffer)?;
                    let deseralized: Block =  serde_json::from_str(buffer.as_str())?;
                    println!("{:?}", deseralized.block_number())
                    //println!("{:?}", serde_json::from_slice(&buffer)?);
                    /*if (buffer[0] == 1) {

                        println!("{:?}", std::str::from_utf8(&buffer).unwrap());
                    } else {
                        println!("{}", "writing");
                        stream.write(b"response")?;
                    }*/
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }
}
