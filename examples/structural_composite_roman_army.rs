use design_patterns::structural::composite::roman_army::{
    primitives::{Archer, Horseman, Infantryman},
    ComponentUnit, CompositeUnit,
};

fn main() {
    let mut army = CompositeUnit::new();

    for _ in 0..4 {
        let legion = create_legion();
        army.add_unit(Box::new(legion));
    }

    {
        let legions = army.get_units();
        println!("Roman army damaging strength is {}", army.get_strength());
        println!("Roman army has {} legions\n", legions.len());
    }

    {
        let legion = army.get_unit(0);
        println!(
            "Roman legion damaging strength is {}",
            legion.get_strength()
        );
        println!("Roman legion has {} units\n", legion.get_units().len());
    }

    army.remove(0);

    let legions = army.get_units();
    println!("Roman army damaging strength is {}", army.get_strength());
    println!("Roman army has {} legions", legions.len());
}

fn create_legion() -> CompositeUnit {
    let mut composite_legion = CompositeUnit::new();

    for _ in 0..3000 {
        composite_legion.add_unit(Box::new(Infantryman));
    }

    for _ in 0..1200 {
        composite_legion.add_unit(Box::new(Archer));
    }

    for _ in 0..300 {
        composite_legion.add_unit(Box::new(Horseman));
    }

    composite_legion
}
