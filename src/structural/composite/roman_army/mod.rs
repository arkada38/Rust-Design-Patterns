pub mod primitives;

pub trait ComponentUnit {
    fn get_strength(&self) -> i32;
    fn add_unit(&mut self, unit: Box<dyn ComponentUnit>);
    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>>;
    fn get_unit(&self, index: usize) -> &dyn ComponentUnit;
    fn remove(&mut self, index: usize);
}

pub struct CompositeUnit {
    units: Vec<Box<dyn ComponentUnit>>,
}

impl CompositeUnit {
    pub fn new() -> CompositeUnit {
        CompositeUnit { units: Vec::new() }
    }
}

impl Default for CompositeUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentUnit for CompositeUnit {
    fn get_strength(&self) -> i32 {
        let mut res = 0;

        for unit in &self.units {
            res += unit.get_strength();
        }

        res
    }

    fn add_unit(&mut self, unit: Box<dyn ComponentUnit>) {
        self.units.push(unit);
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        &self.units
    }

    fn get_unit(&self, index: usize) -> &dyn ComponentUnit {
        &*self.units[index]
    }

    fn remove(&mut self, index: usize) {
        self.units.remove(index);
    }
}
