mod slave;
use slave::*;
use std::env;
fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let port:u16 = arg.parse::<u16>().unwrap();
    let mut slave = Slave::new(port);
    slave.listen_and_serve().unwrap();
}
