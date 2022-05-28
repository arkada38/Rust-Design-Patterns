pub trait Drink {
    fn get_price(&self) -> f32;
    fn get_volume(&self) -> u32;
}

#[derive(Clone, Copy)]
pub struct CheapDrink {
    volume: u32,
}

impl CheapDrink {
    pub fn new(volume: u32) -> CheapDrink {
        CheapDrink { volume }
    }
}

impl Drink for CheapDrink {
    fn get_price(&self) -> f32 {
        4.95
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }
}

#[derive(Clone, Copy)]
pub struct BusinessDrink {
    volume: u32,
}

impl BusinessDrink {
    pub fn new(volume: u32) -> BusinessDrink {
        BusinessDrink { volume }
    }
}

impl Drink for BusinessDrink {
    fn get_price(&self) -> f32 {
        9.95
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }
}
