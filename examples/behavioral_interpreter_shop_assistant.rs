use std::error::Error;
use design_patterns::behavioral::interpreter::shop_assistant::Order;

fn main() -> Result<(), Box<dyn Error>> {
    let order = Order::parse("order x100 'pack of toothpicks' from Tesco")?;
    println!("{:#?}", order);
    
    let order = Order::parse("order x5 'pack of milk' from Macro")?;
    println!("{:#?}", order);

    Ok(())
}
