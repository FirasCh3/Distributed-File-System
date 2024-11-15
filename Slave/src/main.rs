mod slave;
use slave::*;
fn main() {
    let mut slave = Slave::new(4200);
    slave.listen_and_serve().unwrap();
}
