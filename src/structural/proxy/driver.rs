pub struct Driver {
    age: u8,
}

impl Driver {
    pub fn new(age: u8) -> Driver {
        Driver { age }
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}

pub trait Car {
    fn drive_car(&self);
}

pub struct RealCar {}

impl Car for RealCar {
    fn drive_car(&self) {
        println!("Car has been driven!");
    }
}

pub struct ProxyCar {
    driver: Driver,
    real_car: RealCar,
}

impl Car for ProxyCar {
    fn drive_car(&self) {
        if self.driver.age <= 16 {
            println!("Sorry, the driver is too young to drive.")
        } else {
            self.real_car.drive_car();
        }
    }
}

impl ProxyCar {
    pub fn new(driver: Driver) -> ProxyCar {
        ProxyCar {
            driver,
            real_car: RealCar {},
        }
    }
}
