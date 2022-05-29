pub mod garage;
pub mod handler;

pub struct Car {
    km: u32,
    fuel: u8,
    oil: u8,
}

impl Car {
    pub fn new(km: u32, fuel: u8, oil: u8) -> Car {
        Car { km, fuel, oil }
    }
}
