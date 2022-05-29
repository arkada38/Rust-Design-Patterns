pub mod car_factory;
pub mod car_type;

use car_type::CarType;

pub struct Car {
    pub license_plate: String,
    pub parking_place_number: u8,
    pub car_type_id: u8,
}

impl Car {
    pub fn new(license_plate: String, parking_place_number: u8, car_type_id: u8) -> Car {
        Car {
            license_plate,
            parking_place_number,
            car_type_id,
        }
    }

    pub fn print(&self, car_type: &CarType) {
        println!(
            "{} - {:?} {:?}, {}",
            self.parking_place_number, car_type.colour, car_type.body, self.license_plate
        )
    }
}
