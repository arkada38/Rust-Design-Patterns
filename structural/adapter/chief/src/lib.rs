pub mod confectioner;
pub mod chief_adapter;

pub trait Chief {
    fn get_cost(&self) -> u32;
    fn make_dinner(&self);
    fn take_money(&self, money: u32);
}
