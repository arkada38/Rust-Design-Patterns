pub trait Delivery {
    fn issue(&self);
}

pub struct Express {}

impl Delivery for Express {
    fn issue(&self) {
        println!("Super fast delivery for 3 days")
    }
}

pub struct Normal {}

impl Delivery for Normal {
    fn issue(&self) {
        println!("Normal delivery for two weeks")
    }
}
