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
        println!("I'm a robot...");
    }

    fn say_name(&self) {
        println!("My name is {}\n", self.name);
    }
}
