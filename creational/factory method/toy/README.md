# Toy

Sample project with *Factory Method*

## Structure

- Toy
  - Doll
  - Robot

- ToyFactory
  - DollFactory
  - RobotFactory

## Implementation

### Module toy

```rust
pub trait Toy {
    fn sing(&self);
    fn say_name(&self);
}

pub struct Doll {
    pub name: String
}

pub struct Robot {
    pub name: String
}

impl Toy for Doll {
    fn sing(&self) {
        println!("Apple round, apple red...");
    }

    fn say_name(&self) {
        println!("My name is {}\n", self.name);
    }
}

impl Toy for Robot {
    fn sing(&self) {
        println!("I am a robot...");
    }

    fn say_name(&self) {
        println!("My name is {}\n", self.name);
    }
}
```

### Module toy_factory

```rust
pub trait ToyFactory<T> {
    fn create(&self, String) -> T;
}

pub struct DollFactory;
pub struct RobotFactory;

impl ToyFactory<Doll> for DollFactory {
    fn create(&self, name: String) -> Doll {
        Doll { name }
    }
}

impl ToyFactory<Robot> for RobotFactory {
    fn create(&self, name: String) -> Robot {
        Robot { name }
    }
}
```

### main

```rust
fn main() {
    let doll = DollFactory.create("Alena".to_string());
    let robot = RobotFactory.create("Max".to_string());

    play(doll);
    play(robot);
}

fn play<T: Toy>(toy: T) {
    toy.sing();
    toy.say_name();
}
```

### output

```bash
# For Doll
Apple round, apple red...
My name is Alena

# For Robot
I am a robot...
My name is Max
```
