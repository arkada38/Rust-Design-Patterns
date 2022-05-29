use super::Car;

pub trait Handler {
    fn handle(&self, car: &mut Car);
}

pub struct HandleFuel;

impl Handler for HandleFuel {
    fn handle(&self, car: &mut Car) {
        if car.fuel < 10 {
            println!("The car is refueled");
            car.fuel = 100;
        }
    }
}

pub struct HandleKm;

impl Handler for HandleKm {
    fn handle(&self, car: &mut Car) {
        if car.km > 10000 {
            println!("A technical inspection of the car was made");
            car.km = 100;
        }
    }
}

pub struct HandleOil;

impl Handler for HandleOil {
    fn handle(&self, car: &mut Car) {
        if car.oil < 10 {
            println!("Added oil");
            car.oil = 100;
        }
    }
}
