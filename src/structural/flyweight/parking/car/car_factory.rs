use super::car_type::{Body, CarType, Colour};

pub struct CarFactory {
    car_types: Vec<CarType>,
}

impl CarFactory {
    pub fn new() -> CarFactory {
        CarFactory {
            car_types: Vec::new(),
        }
    }

    pub fn get_car_type_id(&mut self, body: Body, colour: Colour) -> u8 {
        let position = self
            .car_types
            .iter()
            .position(|r| r.body == body && r.colour == colour);

        match position {
            Some(x) => x as u8,
            None => {
                let car_type = CarType::new(body, colour);
                self.car_types.push(car_type);
                (self.car_types.len() - 1) as u8
            }
        }
    }

    pub fn get_car_type(&mut self, id: u8) -> Option<&CarType> {
        self.car_types.get(id as usize)
    }

    pub fn print(&self) {
        println!("Number of car types: {}", self.car_types.len());
    }
}

impl Default for CarFactory {
    fn default() -> Self {
        Self::new()
    }
}
