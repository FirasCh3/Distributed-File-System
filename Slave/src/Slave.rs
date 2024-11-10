use std::io::{Read, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
#[derive(Debug)]
pub struct Slave {
    address_read: SocketAddrV4,
    address_write: SocketAddrV4,
    port_read: u16,
    port_write: u16,
}
impl Slave {
    pub fn new(port_read: u16, port_write: u16) -> Slave {
        return Slave {
            address_read: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port_read),
            address_write: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port_write),
            port_read: port_read,
            port_write: port_write,
        };
    }
    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address_read)?;
        for streams in listener.incoming() {
            let mut buffer = [0u8; 1024];
            match streams {
                Ok(mut stream) => {
                    let number_of_bytes = stream.read(&mut buffer)?;
                    println!("{:?}", buffer);
                    if (buffer[0] == 1) {
                        println!("{:?}", std::str::from_utf8(&buffer).unwrap());
                    } else {
                        println!("{}", "doing a write operation");
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
