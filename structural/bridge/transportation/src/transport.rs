use super::delivery::{ Delivery };

pub trait Transport {
    fn load(&self);
    fn set_delivery(&mut self, delivery: Box<Delivery>);
    fn carry(&self);
    fn unload(&self);
}

pub struct Plane {
    delivery: Box<Delivery>
}

impl Plane {
    pub fn new(delivery: Box<Delivery>) -> Plane {
        Plane { delivery }
    }
}

impl Transport for Plane {
    fn load(&self) {
        println!("Loading on the plane... Done!");
    }

    fn set_delivery(&mut self, delivery: Box<Delivery>) {
        self.delivery = delivery;
    }

    fn carry(&self) {
        self.delivery.issue();
        println!("The plane flew off... Done!");
    }

    fn unload(&self) {
        println!("Unloading the plane... Done!\n");
    }
}

pub struct Train {
    delivery: Box<Delivery>
}

impl Train {
    pub fn new(delivery: Box<Delivery>) -> Train {
        Train { delivery }
    }
}

impl Transport for Train{
    fn load(&self) {
        println!("Loading on the train... Done!");
    }

    fn set_delivery(&mut self, delivery: Box<Delivery>) {
        self.delivery = delivery;
    }

    fn carry(&self) {
        self.delivery.issue();
        println!("The train left... Done!");
    }

    fn unload(&self) {
        println!("Unloading the train... Done!\n");
    }
}
