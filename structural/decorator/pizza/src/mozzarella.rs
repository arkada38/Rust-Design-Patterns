use Pizza;

pub struct Mozzarella<'a> {
    base: &'a Pizza,
    description: String,
    cost: f32
}

impl<'a> Mozzarella<'a> {
    pub fn new<T: Pizza>(base:&'a T) -> Mozzarella<'a> {
        Mozzarella {
            base,
            description: "Mozzarella".to_string(),
            cost: 2.49
        }
    }
}

impl<'a> Pizza for Mozzarella<'a> {
    fn get_description(&self) -> String {
        self.base.get_description() + ", " + &self.description
    }

    fn get_cost(&self) -> f32 {
        self.base.get_cost() + self.cost
    }
}
