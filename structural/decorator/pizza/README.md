# Pizza

Sample project with *Decorator*

## Structure

- Pizza
  - Plain pizza
  - Mozzarella
  - Tomato sauce

## Usage

### lib

```rust
pub mod plain_pizza;
pub mod mozzarella;
pub mod tomato_sauce;

pub trait Pizza {
    fn get_description(&self) -> String;
    fn get_cost(&self) -> f32;
}
```

### plain_pizza

```rust
use Pizza;

pub struct PlainPizza {
    description: String,
    cost: f32
}

impl PlainPizza {
    pub fn new() -> PlainPizza {
        PlainPizza {
            description: "Plain pizza".to_string(),
            cost: 5.0
        }
    }
}

impl Pizza for PlainPizza {
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_cost(&self) -> f32 {
        self.cost
    }
}
```

### mozzarella

```rust
use Pizza;

pub struct Mozzarella<'a> {
    base: &'a Pizza,
    description: String,
    cost: f32
}

impl<'a> Mozzarella<'a> {
    pub fn new<T: Pizza>(base:&'a T) -> Mozzarella<'a> {
        Mozzarella {
            base,
            description: "Mozzarella".to_string(),
            cost: 2.49
        }
    }
}

impl<'a> Pizza for Mozzarella<'a> {
    fn get_description(&self) -> String {
        self.base.get_description() + ", " + &self.description
    }

    fn get_cost(&self) -> f32 {
        self.base.get_cost() + self.cost
    }
}
```

### tomato_sauce

```rust
use Pizza;

pub struct TomatoSauce<'a> {
    base: &'a Pizza,
    description: String,
    cost: f32
}

impl<'a> TomatoSauce<'a> {
    pub fn new<T: Pizza>(base:&'a T) -> TomatoSauce<'a> {
        TomatoSauce {
            base,
            description: "Tomato Sauce".to_string(),
            cost: 0.75
        }
    }
}

impl<'a> Pizza for TomatoSauce<'a> {
    fn get_description(&self) -> String {
        self.base.get_description() + ", " + &self.description
    }

    fn get_cost(&self) -> f32 {
        self.base.get_cost() + self.cost
    }
}
```

### main

```rust
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
```

### output

```bash
Description: Plain pizza, Mozzarella, Tomato Sauce
Cost: 8.24
```
