extern crate car;

use car::Car;
use car::handler::{HandleFuel, HandleKm, HandleOil};
use car::garage::Garage;

fn main() {
    let mut car = Car::new(11100, 1,3);

    let mut garage = Garage::new();

    garage.add_handler(HandleFuel {});
    garage.add_handler(HandleOil {});
    garage.add_handler(HandleKm {});

    garage.handle_car(&mut car);
}
