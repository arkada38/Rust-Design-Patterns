use design_patterns::structural::decorator::pizza::{
    mozzarella::Mozzarella, plain_pizza::PlainPizza, tomato_sauce::TomatoSauce, Pizza,
};

fn main() {
    let plain_pizza = PlainPizza::new();
    let mozzarella = Mozzarella::new(&plain_pizza);
    let tomato_sauce = TomatoSauce::new(&mozzarella);

    println!("Description: {}", tomato_sauce.get_description());
    println!("Cost: {}", tomato_sauce.get_cost());
}
