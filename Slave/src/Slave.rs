use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
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
    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address)?;
        for streams in listener.incoming() {
            let mut buffer = [0u8; 1024];
            match streams {
                Ok(mut stream) => loop {
                    let nbytes = stream.read(&mut buffer)?;
                    println!("{:?}", buffer);
                    stream.flush();
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }
}
