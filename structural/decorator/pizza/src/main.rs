extern crate pizza;

use pizza::Pizza;
use pizza::plain_pizza::PlainPizza;
use pizza::mozzarella::Mozzarella;
use pizza::tomato_sauce::TomatoSauce;

fn main() {
    let plain_pizza = PlainPizza::new();
    let mozzarella = Mozzarella::new(&plain_pizza);
    let tomato_sauce = TomatoSauce::new(&mozzarella);

    println!("Description: {}", tomato_sauce.get_description());
    println!("Cost: {}", tomato_sauce.get_cost());
}
