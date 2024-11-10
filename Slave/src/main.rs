mod slave;
use slave::*;
fn main() {
    let mut slave = Slave::new(4200, 4201);
    slave.listen().unwrap();
}
