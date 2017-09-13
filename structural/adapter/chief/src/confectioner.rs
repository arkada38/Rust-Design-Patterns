pub struct Confectioner {}

impl Confectioner {
    pub fn new() -> Confectioner {
        Confectioner{}
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
