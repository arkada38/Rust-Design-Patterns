use super::Pizza;

pub struct TomatoSauce<'a> {
    base: &'a dyn Pizza,
    description: &'a str,
    cost: f32,
}

impl<'a> TomatoSauce<'a> {
    pub fn new<T: Pizza>(base: &'a T) -> TomatoSauce<'a> {
        TomatoSauce {
            base,
            description: "Tomato Sauce",
            cost: 0.75,
        }
    }
}

impl<'a> Pizza for TomatoSauce<'a> {
    fn get_description(&self) -> String {
        format!("{}, {}", self.base.get_description(), &self.description)
    }

    fn get_cost(&self) -> f32 {
        self.base.get_cost() + self.cost
    }
}
