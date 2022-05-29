use super::Pizza;

pub struct PlainPizza<'a> {
    description: &'a str,
    cost: f32,
}

impl PlainPizza<'_> {
    pub fn new<'a>() -> PlainPizza<'a> {
        PlainPizza {
            description: "Plain pizza",
            cost: 5.0,
        }
    }
}

impl Pizza for PlainPizza<'_> {
    fn get_description(&self) -> String {
        self.description.to_string()
    }

    fn get_cost(&self) -> f32 {
        self.cost
    }
}
