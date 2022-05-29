use design_patterns::structural::proxy::driver::{Car, Driver, ProxyCar};

fn main() {
    println!("Case 1. 16 years old driver.");
    let car = ProxyCar::new(Driver::new(16));
    car.drive_car();

    println!();

    println!("Case 1. 21 years old driver.");
    let car = ProxyCar::new(Driver::new(21));
    car.drive_car();
}
