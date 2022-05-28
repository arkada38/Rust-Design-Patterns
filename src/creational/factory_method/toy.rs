pub trait Toy {
    fn sing(&self);
    fn say_name(&self);
}

pub struct Doll {
    pub name: String,
}

pub struct Robot {
    pub name: String,
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

pub trait ToyFactory<T> {
    fn create<S: Into<String>>(&self, name: S) -> T;
}

pub struct DollFactory;
pub struct RobotFactory;

impl ToyFactory<Doll> for DollFactory {
    fn create<S: Into<String>>(&self, name: S) -> Doll {
        Doll { name: name.into() }
    }
}

impl ToyFactory<Robot> for RobotFactory {
    fn create<S: Into<String>>(&self, name: S) -> Robot {
        Robot { name: name.into() }
    }
}
