# Transportation

Sample project with *Bridge*

## Structure

- Transport
  - Plane
  - Trane

- Delivery
  - Express
  - Normal

## Implementation

### Module delivery

```rust
pub trait Delivery {
    fn issue(&self);
}

pub struct Express {}

impl Delivery for Express {
    fn issue(&self) {
        println!("Super fast delivery for 3 days")
    }
}

pub struct Normal {}

impl Delivery for Normal {
    fn issue(&self) {
        println!("Normal delivery for two weeks")
    }
}
```

### Module transport

```rust
pub trait Transport {
    fn download(&self);
    fn set_delivery(&mut self, delivery: Box<Delivery>);
    fn carry(&self);
    fn unload(&self);
}

pub struct Plane {
    delivery: Box<Delivery>
}

impl Plane {
    pub fn new(delivery: Box<Delivery>) -> Plane {
        Plane { delivery }
    }
}

impl Transport for Plane {
    fn download(&self) {
        println!("Downloading on the plane... Done!");
    }

    fn set_delivery(&mut self, delivery: Box<Delivery>) {
        self.delivery = delivery;
    }

    fn carry(&self) {
        self.delivery.issue();
        println!("The plane flew off... Done!");
    }

    fn unload(&self) {
        println!("Unloading the plane... Done!\n");
    }
}

pub struct Train {
    delivery: Box<Delivery>
}

impl Train {
    pub fn new(delivery: Box<Delivery>) -> Train {
        Train { delivery }
    }
}

impl Transport for Train{
    fn download(&self) {
        println!("Downloading on the train... Done!");
    }

    fn set_delivery(&mut self, delivery: Box<Delivery>) {
        self.delivery = delivery;
    }

    fn carry(&self) {
        self.delivery.issue();
        println!("The train left... Done!");
    }

    fn unload(&self) {
        println!("Unloading the train... Done!\n");
    }
}
```

### main

```rust
fn main() {
    let mut plane = Plane::new(Box::new(Express{}));
    plane.download();
    plane.carry();
    plane.unload();

    plane.set_delivery(Box::new(Normal{}));
    plane.download();
    plane.carry();
    plane.unload();

    let mut train = Train::new(Box::new(Express{}));
    train.download();
    train.carry();
    train.unload();

    train.set_delivery(Box::new(Normal{}));
    train.download();
    train.carry();
    train.unload();
}
```

### output

```bash
# By plane with Express delivery
Downloading on the plane... Done!
Super fast delivery for 3 days
The plane flew off... Done!
Unloading the plane... Done!

# By plane with normal delivery
Downloading on the plane... Done!
Normal delivery for two weeks
The plane flew off... Done!
Unloading the plane... Done!

# By trane with Express delivery
Downloading on the train... Done!
Super fast delivery for 3 days
The train left... Done!
Unloading the train... Done!

# By trane with normal delivery
Downloading on the train... Done!
Normal delivery for two weeks
The train left... Done!
Unloading the train... Done!
```
