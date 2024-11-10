use std::io::prelude::*;
use std::net::TcpStream;
fn send() -> std::io::Result<()> {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:4200")?;
    tcp_stream.write_all(&[0]);
    tcp_stream.write_all("abc".as_bytes());
    tcp_stream.flush();
    Ok(())
}
fn main() {
    send();
}
