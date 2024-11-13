use std::io::prelude::*;
use std::net::TcpStream;
use crate::server::Server;

mod server;
fn send() -> std::io::Result<()> {
    /*let mut buffer = [0u8; 1024];
    let mut tcp_stream = TcpStream::connect("127.0.0.1:4200")?;
    tcp_stream.write_all(&[0]);
    tcp_stream.read(&mut buffer);
    println!("{:?}", buffer);*/
    let server: Server = Server::new();
    server.store_file("../resources/test.txt");
    Ok(())
}
fn main() {
    send();
}
