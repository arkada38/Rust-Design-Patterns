pub trait Chief {
    fn get_cost(&self) -> u32;
    fn make_dinner(&self);
    fn take_money(&self, money: u32);
}

pub struct ChiefAdapter {
    confectioner: Confectioner,
}

impl ChiefAdapter {
    pub fn new() -> ChiefAdapter {
        ChiefAdapter {
            confectioner: Confectioner::new(),
        }
    }
}

impl Default for ChiefAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Chief for ChiefAdapter {
    fn get_cost(&self) -> u32 {
        self.confectioner.get_cost_for_dinner()
    }

    fn make_dinner(&self) {
        self.confectioner.make_a_dinner();
    }

    fn take_money(&self, money: u32) {
        self.confectioner.take_money_for_dinner(money);
    }
}

pub struct Confectioner {}

impl Confectioner {
    pub fn new() -> Confectioner {
        Confectioner {}
    }

    pub fn get_cost_for_dinner(&self) -> u32 {
        15
    }

    pub fn make_a_dinner(&self) {
        println!("Confectioner is making a dinner...\nDone!");
    }

    pub fn take_money_for_dinner(&self, money: u32) {
        println!("Thanks for ${}", money);
    }
}

impl Default for Confectioner {
    fn default() -> Self {
        Self::new()
    }
}
