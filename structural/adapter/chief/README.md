# Chief

Sample project with *Adapter*

## Structure

- Chief
  - ChiefAdapter
- Confectioner

## Usage

### lib

```rust
pub mod confectioner;
pub mod chief_adapter;

pub trait Chief {
    fn get_cost(&self) -> u32;
    fn make_dinner(&self);
    fn take_money(&self, money: u32);
}
```

### chief_adapter

```rust
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
```

### confectioner

```rust
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
```

### main

```rust
extern crate chief;

use chief::Chief;
use chief::chief_adapter::ChiefAdapter;

fn main() {
    let chief = ChiefAdapter::new();
    let price = chief.get_cost();
    chief.make_dinner();
    chief.take_money(price + 5);
}
```

### output

```bash
Confectioner is making a dinner...
Done!
Thanks for $20
```
