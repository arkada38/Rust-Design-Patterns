# Parking

Sample project with *Flyweight*

## Structure

- Parking
  - Car
    - CarFactory
    - CarType

## Implementation

### Module car_type

```rust
pub struct CarType {
    pub body: Body,
    pub colour: Colour
}

impl CarType {
    pub fn new(body: Body, colour: Colour) -> CarType {
        CarType { body, colour }
    }
}

#[derive(Debug, PartialEq)]
pub enum Body {
    Sedan,
    Coupe,
    Truck
}

#[derive(Debug, PartialEq)]
pub enum Colour {
    Black,
    Yellow,
    Red
}
```

### Module car_factory

```rust
pub struct CarFactory {
    car_types: Vec<CarType>,
}

impl CarFactory {
    pub fn new() -> CarFactory {
        CarFactory {
            car_types: Vec::new(),
        }
    }

    pub fn get_car_type_id(&mut self, body: Body, colour: Colour) -> u8 {
        let position = self.car_types
            .iter()
            .position(|ref r| r.body == body && r.colour == colour);

        match position {
            Some(x) => x as u8,
            None => {
                let car_type = CarType::new(body, colour);
                self.car_types.push(car_type);
                (self.car_types.len() - 1) as u8
            }
        }
    }

    pub fn get_car_type(&mut self, id: u8) -> Option<&CarType> {
        self.car_types.get(id as usize)
    }

    pub fn print(&self) {
        println!("Number of car types: {}", self.car_types.len());
    }
}
```

### Module car

```rust
pub struct Car {
    pub license_plate: String,
    pub parking_place_number: u8,
    pub car_type_id: u8,
}

impl Car {
    pub fn new(license_plate: String, parking_place_number: u8, car_type_id: u8) -> Car {
        Car {
            license_plate,
            parking_place_number,
            car_type_id,
        }
    }

    pub fn print(&self, car_type: &CarType) {
        println!(
            "{} - {:?} {:?}, {}",
            self.parking_place_number,
            car_type.colour,
            car_type.body,
            self.license_plate
        )
    }
}
```

### Module lib

```rust
pub struct Parking {
    cars: Vec<Car>,
    car_factory: CarFactory,
}

impl Parking {
    pub fn new() -> Parking {
        Parking {
            cars: Vec::new(),
            car_factory: CarFactory::new(),
        }
    }

    pub fn add_car(
        &mut self,
        license_plate: &str,
        parking_place_number: u8,
        body: Body,
        colour: Colour,
    ) {
        self.cars.push(Car::new(
            license_plate.to_string(),
            parking_place_number,
            self.car_factory.get_car_type_id(body, colour),
        ));
    }

    pub fn print(&mut self) {
        for car in &self.cars {
            car.print(self.car_factory.get_car_type(car.car_type_id).unwrap());
        }

        println!("\nNumber of cars: {}", self.cars.len());
        self.car_factory.print();
    }
}
```

### main

```rust
fn main() {
    let mut parking = Parking::new();
    parking.add_car("NFS MW", 13, Body::Coupe, Colour::Red);
    parking.add_car("MW2017", 15, Body::Truck, Colour::Yellow);
    parking.add_car("BIG11", 37, Body::Sedan, Colour::Black);
    parking.add_car("KING", 64, Body::Coupe, Colour::Red);
    parking.add_car("MAN", 73, Body::Coupe, Colour::Red);
    parking.add_car("TE64G", 18, Body::Truck, Colour::Yellow);
    parking.add_car("SMILE", 39, Body::Sedan, Colour::Black);
    parking.add_car("DARK01", 24, Body::Truck, Colour::Black);
    parking.add_car("DARK03", 25, Body::Truck, Colour::Black);
    parking.add_car("DARK05", 26, Body::Truck, Colour::Black);

    parking.print();
}
```

### output

```bash
13 - Red Coupe, NFS MW
15 - Yellow Truck, MW2017
37 - Black Sedan, BIG11
64 - Red Coupe, KING
73 - Red Coupe, MAN
18 - Yellow Truck, TE64G
39 - Black Sedan, SMILE
24 - Black Truck, DARK01
25 - Black Truck, DARK03
26 - Black Truck, DARK05

Number of cars: 10
Number of car types: 4
```
