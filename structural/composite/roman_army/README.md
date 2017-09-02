# Roman Army

Sample project with *Composite*

## Structure

- Component
  - Primitive
    - Archer
    - Infantryman
    - Horseman
  - Composite

## Usage

### main

```rust
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
        println!("Roman legion damaging strength is {}", legion.get_strength());
        println!("Roman legion has {} units\n", legion.get_units().len());
    }

    army.remove(0);

    let legions = army.get_units();
    println!("Roman army damaging strength is {}", army.get_strength());
    println!("Roman army has {} legions", legions.len());
}
```

### output

```bash
Roman army damaging strength is 32400
Roman army has 4 legions

Roman legion damaging strength is 8100
Roman legion has 4500 units

Roman army damaging strength is 24300
Roman army has 3 legions
```
