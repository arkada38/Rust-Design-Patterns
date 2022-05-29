pub struct CarType {
    pub body: Body,
    pub colour: Colour,
}

impl CarType {
    pub fn new(body: Body, colour: Colour) -> CarType {
        CarType { body, colour }
    }
}

#[derive(Debug, PartialEq)]
pub enum Body {
    Sedan,
    Coupe,
    Truck,
}

#[derive(Debug, PartialEq)]
pub enum Colour {
    Black,
    Yellow,
    Red,
}
