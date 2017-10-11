extern crate driver;

use driver::{Car, Driver, ProxyCar};

fn main() {
    let car = ProxyCar::new(Driver::new(16));
    car.drive_car();

    let car = ProxyCar::new(Driver::new(21));
    car.drive_car();
}
