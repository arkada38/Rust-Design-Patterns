use Pizza;

pub struct PlainPizza {
    description: String,
    cost: f32
}

impl PlainPizza {
    pub fn new() -> PlainPizza {
        PlainPizza {
            description: "Plain pizza".to_string(),
            cost: 5.0
        }
    }
}

impl Pizza for PlainPizza {
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_cost(&self) -> f32 {
        self.cost
    }
}
