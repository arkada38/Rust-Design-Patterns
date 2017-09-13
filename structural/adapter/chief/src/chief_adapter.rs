use Chief;
use confectioner::Confectioner;

pub struct ChiefAdapter {
    confectioner: Confectioner
}

impl ChiefAdapter {
    pub fn new() -> ChiefAdapter {
        ChiefAdapter{ confectioner: Confectioner::new() }
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
