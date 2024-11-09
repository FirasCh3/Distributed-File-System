mod Slave;
use Slave::*;

fn main() {
    let mut slave = Slave::new(4200);
    let _ = Slave.listen().unwrap();
}
