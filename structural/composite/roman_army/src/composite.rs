use super::ComponentUnit;

pub struct CompositeUnit {
    units: Vec<Box<ComponentUnit>>
}

impl CompositeUnit {
    pub fn new() -> CompositeUnit {
        CompositeUnit { units: Vec::new() }
    }
}

impl  ComponentUnit  for CompositeUnit  {
    fn get_strength(&self) -> i32  {
        let mut res = 0;

        for unit in &self.units {
            res += unit.get_strength();
        }

        res
    }

    fn add_unit(&mut self, unit: Box<ComponentUnit>) {
        self.units.push(unit);
    }

    fn get_units(&self) -> &Vec<Box<ComponentUnit>> {
        &self.units
    }

    fn get_unit(&self, index: usize) -> &Box<ComponentUnit> {
        &self.units[index]
    }

    fn remove(&mut self, index: usize) {
        self.units.remove(index);
    }
}
