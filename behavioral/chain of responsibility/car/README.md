# Car

Sample project with *Chain of Responsibility*

## Structure

- Car
- Handler
  - HandleFuel
  - HandleKm
  - HandleOil
- Garage

## Implementation

### Module lib

```rust
pub struct Car {
    km: u32,
    fuel: u8,
    oil: u8,
}

impl Car {
    pub fn new(km: u32, fuel: u8, oil: u8) -> Car {
        Car{km, fuel, oil}
    }
}
```

### Module handler

```rust
pub trait Handler {
    fn handle(&self, car: &mut Car);
}

pub struct HandleFuel;

impl Handler for HandleFuel {
    fn handle(&self, car: &mut Car) {
        if car.fuel < 10 {
            println!("The car is refueled");
            car.fuel = 100;
        }
    }
}

pub struct HandleKm;

impl Handler for HandleKm {
    fn handle(&self, car: &mut Car) {
        if car.km > 10000 {
            println!("A technical inspection of the car was made");
            car.km = 100;
        }
    }
}

pub struct HandleOil;

impl Handler for HandleOil {
    fn handle(&self, car: &mut Car) {
        if car.oil < 10 {
            println!("Added oil");
            car.oil = 100;
        }
    }
}
```

### Module garage

```rust
pub struct Garage<'a> {
    handlers: Vec<Box<Handler +'a>>,
}

impl<'a> Garage<'a> {
    pub fn new() -> Garage<'a> {
        Garage{handlers: Vec::new()}
    }

    pub fn add_handler<T>(&mut self, handler: T)
        where T: 'a + Handler {
        self.handlers.push(Box::new(handler));
    }

    pub fn handle_car(&self, car: &mut Car) {
        for handle in &self.handlers {
            handle.handle(car);
        }
    }
}
```

### main

```rust
fn main() {
    let mut car = Car::new(11100, 1,3);

    let mut garage = Garage::new();

    garage.add_handler(HandleFuel {});
    garage.add_handler(HandleOil {});
    garage.add_handler(HandleKm {});

    garage.handle_car(&mut car);
}
```

### output

```bash
The car is refueled
Added oil
A technical inspection of the car was made
```
