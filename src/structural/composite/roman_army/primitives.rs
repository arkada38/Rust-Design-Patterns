use super::ComponentUnit;

pub struct Archer;
pub struct Infantryman;
pub struct Horseman;

impl ComponentUnit for Archer {
    fn get_strength(&self) -> i32 {
        1
    }

    fn add_unit(&mut self, _: Box<dyn ComponentUnit>) {
        unimplemented!()
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        unimplemented!()
    }

    fn get_unit(&self, _: usize) -> &dyn ComponentUnit {
        unimplemented!()
    }

    fn remove(&mut self, _: usize) {
        unimplemented!()
    }
}

impl ComponentUnit for Infantryman {
    fn get_strength(&self) -> i32 {
        2
    }

    fn add_unit(&mut self, _: Box<dyn ComponentUnit>) {
        unimplemented!()
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        unimplemented!()
    }

    fn get_unit(&self, _: usize) -> &dyn ComponentUnit {
        unimplemented!()
    }

    fn remove(&mut self, _: usize) {
        unimplemented!()
    }
}

impl ComponentUnit for Horseman {
    fn get_strength(&self) -> i32 {
        3
    }

    fn add_unit(&mut self, _: Box<dyn ComponentUnit>) {
        unimplemented!()
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        unimplemented!()
    }

    fn get_unit(&self, _: usize) -> &dyn ComponentUnit {
        unimplemented!()
    }

    fn remove(&mut self, _: usize) {
        unimplemented!()
    }
}
