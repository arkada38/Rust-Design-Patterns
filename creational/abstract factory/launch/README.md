# Launch

Sample project with *Abstract Factory*

## Structure

- Launch (*family of related objects*)
  - main_course (*complex object*)
    - weight
    - price
  - drink (*complex object*)
    - volume
    - price

- Launch (*abstract product*)
  - CheapLaunch (*concrete product*)
  - BusinessLaunch (*concrete product*)

- Launch Factory (*abstract factory*)
  - CheapLaunchFactory (*concrete factory*)
  - BusinessLaunchFactory (*concrete factory*)

## Implementation

### Module launch

```rust
pub trait Launch<M: MainCourse, D: Drink> {
    fn get_main_course(&self) -> M;
    fn get_drink(&self) -> D;
}

pub struct CheapLaunch {
    pub main_course: CheapCourse,
    pub drink: CheapDrink
}

impl CheapLaunch {
    pub fn new() -> CheapLaunch {
        CheapLaunch {
            main_course: CheapCourse::new(110),
            drink: CheapDrink::new(50)
        }
    }
}

impl Launch<CheapCourse, CheapDrink> for CheapLaunch {
    fn get_main_course(&self) -> CheapCourse {
        self.main_course
    }

    fn get_drink(&self) -> CheapDrink {
        self.drink
    }
}

pub struct BusinessLaunch {
    pub main_course: BusinessCourse,
    pub drink: BusinessDrink
}

impl BusinessLaunch {
    pub fn new() -> BusinessLaunch {
        BusinessLaunch {
            main_course: BusinessCourse::new(1500),
            drink: BusinessDrink::new(250)
        }
    }
}

impl Launch<BusinessCourse, BusinessDrink> for BusinessLaunch {
    fn get_main_course(&self) -> BusinessCourse {
        self.main_course
    }

    fn get_drink(&self) -> BusinessDrink {
        self.drink
    }
}
```

### Module drink

```rust
pub trait Drink {
    fn get_price(&self) -> f32;
    fn get_volume(&self) -> u32;
}

#[derive(Clone, Copy)]
pub struct CheapDrink {
    volume: u32
}

impl CheapDrink {
    pub fn new(volume: u32) -> CheapDrink {
        CheapDrink { volume }
    }
}

impl Drink for CheapDrink {
    fn get_price(&self) -> f32 {
        4.95
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }
}

#[derive(Clone, Copy)]
pub struct BusinessDrink {
    volume: u32
}

impl BusinessDrink {
    pub fn new(volume: u32) -> BusinessDrink {
        BusinessDrink { volume }
    }
}

impl Drink for BusinessDrink {
    fn get_price(&self) -> f32 {
        9.95
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }
}
```

### Module main_course

```rust
pub trait MainCourse {
    fn get_price(&self) -> f32;
    fn get_weight(&self) -> u32;
}

#[derive(Clone, Copy)]
pub struct CheapCourse {
    weight: u32
}

impl CheapCourse {
    pub fn new(weight: u32) -> CheapCourse {
        CheapCourse { weight }
    }
}

impl MainCourse for CheapCourse {
    fn get_price(&self) -> f32 {
        9.99
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }
}

#[derive(Clone, Copy)]
pub struct BusinessCourse {
    weight: u32
}

impl BusinessCourse {
    pub fn new(weight: u32) -> BusinessCourse {
        BusinessCourse { weight }
    }
}

impl MainCourse for BusinessCourse {
    fn get_price(&self) -> f32 {
        19.99
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }
}
```

### Module launch_factory

```rust
pub trait LaunchFactory<T> {
    fn create(&self) -> T;
}

pub struct CheapLaunchFactory;
pub struct BusinessLaunchFactory;

impl LaunchFactory<CheapLaunch> for CheapLaunchFactory {
    fn create(&self) -> CheapLaunch {
        CheapLaunch::new()
    }
}

impl LaunchFactory<BusinessLaunch> for BusinessLaunchFactory {
    fn create(&self) -> BusinessLaunch {
        BusinessLaunch::new()
    }
}
```

### main

```rust
fn main() {
    //Creation of concrete factory
    let cheap_launch = CheapLaunchFactory.create();
    //Working with factory
    print_prices(cheap_launch);

    //Creation of concrete factory
    let business_launch = BusinessLaunchFactory.create();
    //Working with factory
    print_prices(business_launch);
}

fn print_prices<M, D, L>(launch: L)
    where M: MainCourse,
          D: Drink,
          L: Launch<M, D> {
    let maine_course = launch.get_main_course();
    let drink = launch.get_drink();

    print_main_course_price(maine_course);
    print_drink_price(drink);
}

fn print_main_course_price<M: MainCourse>(main_course: M) {
    println!("Main Course: {}$", main_course.get_price());
}

fn print_drink_price<D: Drink>(drink: D) {
    println!("Drink: {}$\n", drink.get_price());
}
```

### output

```bash
# For CheapLaunch
Main Course: 9.99$
Drink: 4.95$

# For BusinessLaunch
Main Course: 19.99$
Drink: 9.95$
```
