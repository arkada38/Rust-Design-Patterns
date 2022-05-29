pub mod mozzarella;
pub mod plain_pizza;
pub mod tomato_sauce;

pub trait Pizza {
    fn get_description(&self) -> String;
    fn get_cost(&self) -> f32;
}
