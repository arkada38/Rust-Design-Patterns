pub mod primitives;
pub mod composite;

pub trait ComponentUnit  {
    fn get_strength(&self) -> i32;
    fn add_unit(&mut self, unit: Box<ComponentUnit>);
    fn get_units(&self) -> &Vec<Box<ComponentUnit>>;
    fn get_unit(&self, index: usize) -> &Box<ComponentUnit>;
    fn remove(&mut self, index: usize);
}
